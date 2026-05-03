api-investo/
├── src/
│   ├── main.rs          # Point d'entrée, initialisation du serveur
│   ├── config.rs        # Gestion des .env et de la config
│   ├── routes/          # Définition des points d'entrée
│   │   ├── mod.rs       # Agrégation des routes
│   │   └── pokemon.rs   # Routes liées aux Pokémon
│   ├── handlers/        # Fonctions de traitement (Logique métier)
│   │   ├── mod.rs
│   │   └── pokemon.rs
│   └── models/          # Structs Serde et SQLx
│       ├── mod.rs
│       └── pokemon.rs
├── .env
└── Cargo.toml# api-rust-investo
