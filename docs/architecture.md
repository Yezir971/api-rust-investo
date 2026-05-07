api-investo/
├── src/
│   ├── main.rs          # Point d'entrée, initialisation du serveur
│   ├── routes/          # Définition des points d'entrée
│   │   ├── bot.rs       # 
│   │   ├── mod.rs       # Agrégation des routes
│   │   └── user.rs      # Routes liées aux utilisateurs
│   ├── handlers/        # Fonctions de traitement (Logique métier)
│   │   ├── bot.rs
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── middleware/      # 
│   │   ├── auth.rs
│   │   └── mod.rs
│   ├── schema/          # 
│   │   ├── bot.rs
│   │   ├── jwt.rs
│   │   ├── state.rs
│   │   ├── user.rs
│   │   └── mod.rs
│   ├── utils/           #
│   │   ├── utils.rs
│   │   └── mod.rs
│   └── models/          # Structs Serde et SQLx
│       ├── mod.rs
│       └── user.rs
├── migrations/
├── docs/
│   ├── architectur.dm
│   ├── plan.dm
│   └── adr/
|       ├── 0001_choix_langage_back.md

├── .env.example     # prévisualisation des données à trasnmettre dans le .env
├── .env
└── Cargo.toml# api-rust-investo

