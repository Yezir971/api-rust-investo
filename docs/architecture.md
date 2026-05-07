api-investo/
├── src/
│   ├── main.rs          # Point d'entrée, initialisation du serveur
│   ├── config.rs        # Gestion des .env et de la config
│   ├── routes/          # Définition des points d'entrée
│   │   ├── mod.rs       # Agrégation des routes
│   │   └── user.rs      # Routes liées aux utilisateurs
│   ├── handlers/        # Fonctions de traitement (Logique métier)
│   │   ├── mod.rs
│   │   └── user.rs
│   └── models/          # Structs Serde et SQLx
│       ├── mod.rs
│       └── user.rs
├── .env
└── Cargo.toml# api-rust-investo

