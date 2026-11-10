# ADR-004 — Chiffrement de bout en bout : MLS (OpenMLS)
**Statut** : Accepté — spike du Lot 0 concluant (voir `documentation/architecture/01-spike-mls.md`), avec réserves listées dans ce rapport (état non persistant, WASM compilé mais non exécuté, pas de vecteurs officiels)
**Contexte** : conversations privées E2EE par défaut ; produit propriétaire ; pas de protocole maison. `libsignal` est (à vérifier au moment de l'adoption) sous AGPL-3.0, incompatible avec un produit fermé sans accord.
**Décision** : MLS (RFC 9420) via OpenMLS pour conversations directes (groupe de deux) et groupes ; un appareil = une feuille de l'arbre. Le serveur est annuaire de key packages et relais ordonné, sans aucune clé privée.
**Alternatives** : Signal/libsignal (licence) ; Olm/Megolm via vodozemac (repli, Apache-2.0, sécurité de groupe plus faible) ; maison (interdit).
**Conséquences** : bibliothèque plus jeune que Signal ; **aucun audit externe prévu (budget nul)** → le produit est « non audité » et ne doit pas être présenté comme sûr (voir SECURITY.md). Mesures compensatoires : vecteurs de test, fuzzing, revue interne.
**Critères du spike** : deux clients échangent un message ; ajout/retrait de membre ; build natif et WASM ; vérification de la licence et de l'activité du projet ; vecteurs de test.
