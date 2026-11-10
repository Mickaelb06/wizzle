# CLAUDE.md — règles de travail sur WIZZLE

Équipe : 1 humain + Claude. Langue des docs et commits : français.

## Règles
- Lire `documentation/architecture/00-plan-execution.md` et `documentation/decisions/` avant toute décision structurante.
- Ne pas écraser le travail existant ; garder l'application exécutable à chaque étape.
- **Jamais de cryptographie maison.** Bibliothèques reconnues uniquement ; toute dépendance crypto passe par un ADR.
- Pas de secrets dans le dépôt. Pas de dépendance lourde sans justification (licence vérifiée : `cargo deny`).
- Pas de fonctionnalité factice présentée comme terminée ; tout élément provisoire est marqué `PROVISOIRE`.
- Chaque changement : compilation + tests + analyse statique (clippy) + vérification fonctionnelle. Les parties crypto/sécurité sont signalées comme **non auditées**.
- Mettre à jour la documentation à chaque changement architectural.
- Un module à la fois ; ordre : fondations → identité → messagerie → sync.

## Commandes
À compléter au Lot 1 (build, test, lint).
