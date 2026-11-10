# SECURITY

## Signaler une vulnérabilité
Canal de signalement à définir avant toute bêta (adresse dédiée + politique de divulgation).

## Statut honnête de la sécurité
- **Aucun audit externe** n'est prévu à ce stade (budget nul). Le code cryptographique et d'authentification est donc **non audité** : il ne doit pas être présenté comme « sûr » ou « audité » publiquement.
- Mesures compensatoires : bibliothèques reconnues uniquement, vecteurs de test officiels, fuzzing, revue interne dédiée, CI (`cargo audit`, `cargo deny`), tests d'autorisation négatifs.
- Bêta limitée à des utilisateurs informés, avec mention explicite « non audité ».
- Les métadonnées (qui, quand, taille) sont visibles du serveur ; seul le contenu des messages privés est chiffré de bout en bout.

Modèle détaillé : voir plan §8 et ADR-004.
