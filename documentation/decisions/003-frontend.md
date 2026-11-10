# ADR-003 — Frontend
**Statut** : Accepté, réversible, sous réserve du spike du Lot 0
**Contexte** : équipe = 1 humain + Claude ; le Web doit être livrable, avec un niveau d'assurance E2EE documenté (peut suivre le mobile).
**Décision** : **Flutter pour Android, iOS et Web** au départ (une seule base UI). Le chiffrement et la synchronisation vivent dans un **noyau client Rust** (`client-core/`) exposé par FFI (mobile) et WASM (Web), pour que le choix de l'UI reste réversible.
**Alternatives** : Flutter mobile + React/TS Web (meilleur Web mais deux UI : irréaliste à deux) ; React Native ; natif ; WebView. Comparatif complet : plan §5.
**Conséquences** : Web de moindre qualité (poids, accessibilité, SEO) ; si insuffisant, remplacer l'UI Web par React/TS sans toucher au noyau. Design en tokens (JSON).
**Condition de validation** : spike mesurant poids initial, fluidité d'une longue liste, accessibilité lecteur d'écran, intégration WASM du noyau.
