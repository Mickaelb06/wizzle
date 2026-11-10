# ADR-001 — Monolithe modulaire
**Statut** : Accepté
**Contexte** : équipe d'une personne + IA, budget nul, besoin de simplicité et d'évolutivité.
**Décision** : un seul binaire Rust, workspace Cargo avec un crate par module (`identity`, `messaging`, `media`, `notifications`, `safety`, `platform`). Un schéma PostgreSQL par module, pas de jointure inter-schémas, dépendances entre modules via traits publics uniquement.
**Alternatives** : microservices (surcoût d'exploitation, rejeté par le cahier) ; monolithe sans frontières (extraction future coûteuse).
**Conséquences** : déploiement simple ; discipline de frontières à vérifier en CI ; extraction possible module par module.
