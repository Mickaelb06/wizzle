# ADR-005 — Authentification
**Statut** : Accepté
**Décision** : sessions propriétaires : jeton d'accès court + refresh token opaque rotatif lié à l'appareil, stocké haché, détection de réutilisation ; mots de passe Argon2id ; passkeys (`webauthn-rs`) ; TOTP en MFA ; rate limiting. OIDC/OAuth 2.1 uniquement pour les tiers (Phase 4).
**Alternatives** : fournisseur OIDC dès le départ (complexité inutile pour les clients propriétaires) ; service managé (coût, dépendance).
**Conséquences** : révocation et déconnexion à distance simples ; à étendre avec OIDC pour mini-apps.
