# Lot 0 — Spike chiffrement MLS (OpenMLS)

Code : `spikes/mls/` (jetable). Date : 2026-11-10. Verdict : **concluant pour continuer ; ADR-004 confirmé avec réserves.**

## Ce qui a été vérifié (exécuté, 10 contrôles OK)
| Contrôle | Résultat |
|---|---|
| Création de groupe, invitation par key package → Welcome | OK |
| Le « serveur » (relais d'octets) ne voit pas le texte clair ni les identités dans le message chiffré | OK |
| Déchiffrement par le destinataire ; rejeu du même message rejeté | OK |
| **Multi-appareils** : un 2e appareil de Bob rejoint comme feuille supplémentaire ; les deux déchiffrent | OK |
| Groupe de 4 appareils (3 personnes) ; liste de membres cohérente | OK |
| **Retrait** d'un appareil : il ne peut plus lire les messages suivants, les autres si | OK |
| Compilation **native** | OK |
| Compilation **WASM** (`wasm32-unknown-unknown`) | OK (features `js` sur openmls et `getrandom` 0.2/0.4 requises) |
| Taille WASM release du scénario complet | ≈ 1,5 Mo, ≈ 0,5 Mo gzip (`opt-level=z`, LTO) |

## Licences et maturité
- `openmls` 0.9.2 : licence **MIT**, publié le 2026-10-22, ~1,2 M de téléchargements.
- Arbre de dépendances : 197 paquets, aucun GPL/AGPL/SSPL en licence unique ; un paquet (`r-efi`) propose `MIT OR Apache-2.0 OR LGPL` (on choisit MIT). Compatible avec un produit propriétaire. À automatiser avec `cargo deny` (Lot 1).

## Limites de ce spike (honnêteté)
- Le scénario n'a été **exécuté qu'en natif** ; le WASM n'a été que **compilé**, pas exécuté dans un navigateur/Node.
- Stockage en mémoire uniquement : pas de persistance des clés, donc rien sur la sécurité du stockage (Keychain/Keystore/WebCrypto), la reprise après redémarrage ni la migration d'état.
- Aucun réseau, aucun ordonnancement serveur des commits (conflits entre commits concurrents non testés), aucune expiration/rotation de key packages.
- Un seul ciphersuite testé (X25519/AES-128-GCM/Ed25519), identifiants `BasicCredential` (pas d'authentification d'identité ni de vérification de clés entre utilisateurs).
- Pas de fuzzing, pas de vecteurs de test officiels MLS exécutés ici. Pas de mesure de performance.
- Ni le code ni la bibliothèque n'ont fait l'objet d'un audit par WIZZLE ; pas d'audit externe prévu (budget nul).

## Points à traiter aux lots suivants
1. **Lot 3** : persistance chiffrée de l'état (SQLite côté client), ordonnancement des commits par le serveur, gestion des commits concurrents, key packages à usage unique + dernier recours, reprise après coupure.
2. **Lot 4** : ajout d'appareil approuvé par un appareil existant, vérification de clés (numéro de sécurité), sauvegarde utilisateur.
3. Exécuter le noyau en WASM réel (navigateur) et mesurer le coût.
4. Exécuter les vecteurs de test MLS officiels et mettre en place le fuzzing.
