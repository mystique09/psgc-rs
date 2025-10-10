# PSGC-rs

[!NOTE]
**🚧 WORK IN PROGRESS** - This project is currently under active development. APIs may change, and some features might not be fully implemented yet.

An over-engineered Rust implementation of the Philippine Standard Geographic Code (PSGC) API. This project serves as a rewrite of the original PSGC-API, leveraging Rust's performance and safety features along with modern database technologies.

## 🚀 Features

- **Hierarchical Geographic Data Management**: Full support for Philippine administrative divisions (Regions → Provinces → Cities/Municipalities → Barangays)
- **High-Performance API**: Built with Actix-web for blazing fast responses
- **Database Support**: PostgreSQL with Redis caching
- **Modular Architecture**: Clean separation of concerns using workspace crates
- **Migration Management**: Built-in database migration tools
- **Data Seeding**: Automated PSGC data seeding functionality

## 🏗️ Architecture

This project follows a modular architecture with the following workspace crates:

- **`psgc-api`**: HTTP API layer, controllers, and routing using Actix-web
- **`psgc-application`**: Application services, use cases, and DTOs
- **`psgc-domain`**: Core domain entities and business logic
- **`psgc-infrastructure`**: Database connections, external services, and technical infrastructure
- **`psgc-shared`**: Common utilities and shared types across the project

## 🛠️ Prerequisites

Before running this project, ensure you have the following tools installed:

- **Rust** (latest stable version)
- **PostgreSQL** (version 13 or higher)
- **Redis** (for caching)
- **Docker & Docker Compose** (for containerized development)

## 🚀 Quick Start

### Using Docker Compose (Recommended)

1. **Clone and set up the project:**
   ```bash
   git clone <repository-url>
   cd psgc-rs
   cp .env.example .env
   ```

2. **Start the development environment:**
   ```bash
   docker-compose up -d
   ```

3. **Run database migrations:**
   ```bash
   cargo run --bin migrator --release
   ```

4. **Seed the database with PSGC data:**
   ```bash
   cargo run --bin seeder --release
   ```

5. **Start the API server:**
   ```bash
   cargo run --release
   ```

### Manual Setup

If you prefer to run PostgreSQL and Redis locally:

1. **Install dependencies** as listed above
2. **Set up environment variables** in a `.env` file:
   ```env
   DATABASE_URL=postgresql://postgres:secret@localhost:5432/psgc-rs?sslmode=disabled
   REDIS_URL=redis://localhost:6379
   ```

3. **Run the same commands** as in the Docker setup

## 📁 Project Structure

```
psgc-rs/
├── crates/
│   ├── psgc-api/             # HTTP API layer, controllers, and routing
│   ├── psgc-application/     # Application services, use cases, and DTOs
│   ├── psgc-domain/          # Domain entities and business logic
│   ├── psgc-infrastructure/  # Database and external service integrations
│   └── psgc-shared/          # Shared utilities and types
├── src/
│   └── bin/
│       ├── main.rs           # API server entry point
│       ├── migrator.rs      # Database migration runner
│       └── seeder.rs        # Data seeding utility
├── docker-compose.yml       # Development environment setup
├── Cargo.toml               # Workspace configuration
└── README.md               # This file
```

## 🗄️ Database Schema

The project manages PSGC data with a hierarchical structure:
- **Regions**: Top-level administrative divisions
- **Provinces**: Subdivisions within regions
- **Cities/Municipalities**: Subdivisions within provinces
- **Barangays**: Smallest administrative units

The PSGC code system uses a 9-digit hierarchical format: `RRPPMMBBB`
where relationships are encoded numerically for efficient querying.

## 🧪 Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Building for Production

```bash
cargo build --release
```

## 📚 PSGC Information

The Philippine Standard Geographic Code (PSGC) is a systematic classification and coding of geographic areas in the Philippines maintained by the Philippine Statistics Authority. Each administrative unit is assigned a unique 9-digit code that maintains hierarchical relationships.

## 🤝 Contributing

Contributions are welcome! Please ensure:
- Code follows Rust conventions
- Tests are included for new features
- Documentation is updated as needed
- All checks pass before submitting

## 🔗 Related Resources

- [Philippine Statistics Authority - PSGC](https://psa.gov.ph/classification/psgc)
- [Original PSGC-API](https://gitlab.com/psgc/api)
- [PSGC Documentation](https://psgc.cloud)