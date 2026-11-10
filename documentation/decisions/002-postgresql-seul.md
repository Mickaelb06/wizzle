# ADR-002 — PostgreSQL seul au MVP
**Statut** : Accepté
**Contexte** : le cahier liste Redis, NATS JetStream et OpenSearch.
**Décision** : MVP = PostgreSQL (données, files de livraison par appareil, rate limiting, recherche utilisateur via `pg_trgm`) + stockage S3-compatible. Redis, NATS, OpenSearch reportés avec déclencheurs mesurables (saturation du limiteur/présence ; multi-instance ; recherche publique avancée).
**Alternatives** : tout installer dès le départ (coût d'exploitation sans besoin avéré).
**Conséquences** : moins de composants à exploiter ; la file de livraison en base devra être mesurée en charge (Lot 8).
