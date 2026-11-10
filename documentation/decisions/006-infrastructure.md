# ADR-006 — Hébergement et infrastructure
**Statut** : Accepté
**Décision** : VPS Linux + Docker Compose + Caddy ; stockage objet S3-compatible ; sauvegardes chiffrées hors site ; CI GitHub Actions. Hébergement UE par défaut (non obligatoire). Pas de Kubernetes.
**Conséquences** : coût minimal (ordre de 20–200 €/mois, à confirmer) ; exploitation manuelle assumée ; test de restauration à chaque lot.
