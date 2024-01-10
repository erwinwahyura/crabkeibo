# crabkeibo
its a kakeibo apps building with rust


# Structure 
```
crabkeibo/
├── Cargo.toml
├── src/
│   ├── main.rs             # Main entry point for the binary
│   ├── lib.rs              # If you have common library code
│   ├── api/                # API specific modules
│   │   ├── mod.rs          # Module declaration for api
│   │   ├── routes.rs       # API routes and endpoint definitions
│   │   └── handlers.rs     # Request handlers
│   ├── models/             # Data models and structures
│   │   ├── mod.rs          # Module declaration for models
│   │   └── user.rs         # Example model (e.g., User)
│   ├── services/           # Business logic
│   │   ├── mod.rs          # Module declaration for services
│   │   └── user_service.rs # Example service (e.g., User service)
│   └── utils/              # Utility functions and common helpers
│       ├── mod.rs          # Module declaration for utils
│       └── db.rs           # Database utilities, for example
└── .env                    # Environment variables (if needed)
```