src/
├── main.rs           # Application entry point
├── config.rs         # Configuration handling
├── routes.rs         # Route definitions
├── handlers/         # Request handlers
│   ├── mod.rs       # Handler module exports
│   ├── auth.rs      # Authentication handlers
│   ├── users.rs     # User-related handlers
│   └── products.rs  # Product-related handlers
├── models/          # Database models
│   ├── mod.rs      # Model module exports
│   ├── user.rs     # User model
│   └── product.rs  # Product model
├── middleware/      # Custom middleware
│   ├── mod.rs      # Middleware module exports
│   └── auth.rs     # Authentication middleware
└── db/             # Database related code
    ├── mod.rs      # Database module exports
    └── pool.rs     # Database pool setup