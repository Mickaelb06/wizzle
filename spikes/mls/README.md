# Spike MLS (Lot 0) — jetable

Valide l'ADR-004 (OpenMLS). **Code jetable, non destiné à la production, non audité.**

    cargo run                                              # scénario complet, 10 contrôles
    cargo build --release --target wasm32-unknown-unknown  # compilation WASM (nécessite le target)

Résultats : `documentation/architecture/01-spike-mls.md`.
