# ADR-007 — Protocole de synchronisation
**Statut** : Proposé (spécification détaillée au Lot 3)
**Décision proposée** : séquence monotone par appareil destinataire ; curseur acquitté par appareil ; envoi idempotent via `client_message_id` ; WebSocket + repli `GET /sync?since=` ; le serveur supprime le chiffré après acquittement de tous les appareils actifs ou expiration ; présence/saisie éphémères.
**À préciser** : expiration, gestion des appareils inactifs, ordonnancement des commits MLS, taille des lots, semantics de reprise.
