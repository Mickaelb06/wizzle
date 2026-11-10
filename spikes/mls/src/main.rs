//! SPIKE JETABLE — Lot 0, ADR-004 : valider OpenMLS pour WIZZLE.
//!
//! Scénario : le « serveur » ne manipule que des octets opaques (relais).
//!  1. Alice crée un groupe, invite Bob (key package -> Welcome).
//!  2. Alice -> Bob : message chiffré ; le serveur ne voit pas le clair.
//!  3. Bob ajoute un 2e appareil (« bob-tel » + « bob-pc » = 2 feuilles) : multi-appareils.
//!  4. Alice ajoute Carol (groupe de 3+).
//!  5. Retrait de l'appareil perdu : il ne peut plus lire les messages suivants.
//!  6. Rejeu d'un même message : rejeté.
use openmls::prelude::{tls_codec::*, *};
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;

const CS: Ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

struct Device {
    name: &'static str,
    provider: OpenMlsRustCrypto,
    signer: SignatureKeyPair,
    cred: CredentialWithKey,
    group: Option<MlsGroup>,
}

impl Device {
    fn new(name: &'static str) -> Self {
        let provider = OpenMlsRustCrypto::default();
        let signer = SignatureKeyPair::new(CS.signature_algorithm()).unwrap();
        signer.store(provider.storage()).unwrap();
        let cred = CredentialWithKey {
            credential: BasicCredential::new(name.as_bytes().to_vec()).into(),
            signature_key: signer.public().into(),
        };
        Device { name, provider, signer, cred, group: None }
    }
    /// Key package publié sur le « serveur » (octets).
    fn key_package(&self) -> Vec<u8> {
        let kp = KeyPackage::builder()
            .build(CS, &self.provider, &self.signer, self.cred.clone())
            .unwrap();
        MlsMessageOut::from(kp.key_package().clone()).tls_serialize_detached().unwrap()
    }
    fn create_group(&mut self) {
        let cfg = MlsGroupCreateConfig::builder().ciphersuite(CS).use_ratchet_tree_extension(true).build();
        self.group = Some(MlsGroup::new(&self.provider, &self.signer, &cfg, self.cred.clone()).unwrap());
    }
    /// Retourne (commit, welcome) en octets.
    fn add(&mut self, kp_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let kp = match MlsMessageIn::tls_deserialize_exact(kp_bytes).unwrap().extract() {
            MlsMessageBodyIn::KeyPackage(k) => k.validate(self.provider.crypto(), ProtocolVersion::Mls10).unwrap(),
            _ => panic!("pas un key package"),
        };
        let g = self.group.as_mut().unwrap();
        let (commit, welcome, _) = g.add_members(&self.provider, &self.signer, &[kp]).unwrap();
        g.merge_pending_commit(&self.provider).unwrap();
        (commit.tls_serialize_detached().unwrap(), welcome.tls_serialize_detached().unwrap())
    }
    fn join(&mut self, welcome: &[u8]) {
        let w = match MlsMessageIn::tls_deserialize_exact(welcome).unwrap().extract() {
            MlsMessageBodyIn::Welcome(w) => w,
            _ => panic!("pas un welcome"),
        };
        let cfg = MlsGroupJoinConfig::builder().use_ratchet_tree_extension(true).build();
        let staged = StagedWelcome::new_from_welcome(&self.provider, &cfg, w, None).unwrap();
        self.group = Some(staged.into_group(&self.provider).unwrap());
    }
    fn send(&mut self, text: &str) -> Vec<u8> {
        let g = self.group.as_mut().unwrap();
        g.create_message(&self.provider, &self.signer, text.as_bytes()).unwrap().tls_serialize_detached().unwrap()
    }
    /// Reçoit un message de protocole (commit ou application). Retourne le clair si application.
    fn receive(&mut self, bytes: &[u8]) -> Result<Option<String>, String> {
        let msg = MlsMessageIn::tls_deserialize_exact(bytes).map_err(|e| e.to_string())?;
        let proto = msg.try_into_protocol_message().map_err(|e| e.to_string())?;
        let g = self.group.as_mut().ok_or("pas de groupe")?;
        let p = g.process_message(&self.provider, proto).map_err(|e| e.to_string())?;
        match p.into_content() {
            ProcessedMessageContent::ApplicationMessage(m) => {
                Ok(Some(String::from_utf8(m.into_bytes()).unwrap()))
            }
            ProcessedMessageContent::StagedCommitMessage(c) => {
                g.merge_staged_commit(&self.provider, *c).map_err(|e| e.to_string())?;
                Ok(None)
            }
            _ => Ok(None),
        }
    }
    fn remove(&mut self, who: &str) -> Vec<u8> {
        let g = self.group.as_mut().unwrap();
        let idx = g.members().find(|m| {
            BasicCredential::try_from(m.credential.clone()).unwrap().identity() == who.as_bytes()
        }).unwrap().index;
        let (commit, _, _) = g.remove_members(&self.provider, &self.signer, &[idx]).unwrap();
        g.merge_pending_commit(&self.provider).unwrap();
        commit.tls_serialize_detached().unwrap()
    }
    fn members(&self) -> Vec<String> {
        self.group.as_ref().unwrap().members()
            .map(|m| String::from_utf8(BasicCredential::try_from(m.credential).unwrap().identity().to_vec()).unwrap())
            .collect()
    }
}

fn check(label: &str, ok: bool) {
    println!("[{}] {label}", if ok { "OK" } else { "ÉCHEC" });
    assert!(ok, "{label}");
}

fn main() {
    let secret = "rendez-vous secret à 18h";
    let mut alice = Device::new("alice");
    let mut bob_tel = Device::new("bob-tel");
    let mut bob_pc = Device::new("bob-pc");
    let mut carol = Device::new("carol");

    // 1. création + invitation (le « serveur » relaie des octets)
    alice.create_group();
    let (_commit, welcome) = alice.add(&bob_tel.key_package());
    bob_tel.join(&welcome);
    check("Bob rejoint via Welcome", bob_tel.members().len() == 2);

    // 2. message chiffré, opaque pour le serveur
    let ct = alice.send(secret);
    check("le serveur ne voit pas le clair", !ct.windows(secret.len()).any(|w| w == secret.as_bytes()));
    check("Bob déchiffre", bob_tel.receive(&ct).unwrap().as_deref() == Some(secret));
    check("rejeu du même message rejeté", bob_tel.receive(&ct).is_err());

    // 3. multi-appareils : Bob ajoute son PC (une feuille de plus)
    let (commit, welcome) = bob_tel.add(&bob_pc.key_package());
    alice.receive(&commit).unwrap();
    bob_pc.join(&welcome);
    let ct = alice.send("message pour tous les appareils");
    check("2 appareils de Bob déchiffrent",
        bob_tel.receive(&ct).unwrap().is_some() && bob_pc.receive(&ct).unwrap().is_some());

    // 4. 3e participant
    let (commit, welcome) = alice.add(&carol.key_package());
    bob_tel.receive(&commit).unwrap();
    bob_pc.receive(&commit).unwrap();
    carol.join(&welcome);
    let ct = carol.send("salut de Carol");
    check("groupe de 4 appareils fonctionne",
        alice.receive(&ct).unwrap().is_some() && bob_tel.receive(&ct).unwrap().is_some()
        && bob_pc.receive(&ct).unwrap().is_some());
    check("liste des membres cohérente", alice.members().len() == 4);

    // 5. retrait de bob-tel (téléphone perdu) -> post-compromise : plus d'accès
    let commit = alice.remove("bob-tel");
    bob_pc.receive(&commit).unwrap();
    carol.receive(&commit).unwrap();
    let _ = bob_tel.receive(&commit); // l'appareil retiré reçoit le commit mais ne peut plus suivre
    let ct = alice.send("après retrait");
    let r = bob_tel.receive(&ct);
    println!("   (réception par l'appareil retiré : {:?})", r.as_ref().map(|o| o.is_some()).map_err(|e| e.as_str()));
    check("appareil retiré ne peut plus lire", !matches!(r, Ok(Some(_))));
    check("les autres lisent toujours", bob_pc.receive(&ct).unwrap().as_deref() == Some("après retrait"));

    // 6. ancien état : le nom des participants n'est jamais exposé en clair dans le chiffré
    check("identités absentes du message chiffré", !ct.windows(5).any(|w| w == b"alice"));
    println!("Spike MLS terminé : tous les contrôles passent. ({})", alice.name);
}
