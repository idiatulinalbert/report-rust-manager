# First Project - Rust Web API

A robust and modern web API built with **Rust**, **Axum**, and **SQLite**. This project demonstrates a production-ready REST API with user authentication, order management, and comprehensive database operations.

![Project Banner](./docs/images/banner.png)

---

## 📋 Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Application](#running-the-application)
- [API Endpoints](#api-endpoints)
- [Database Schema](#database-schema)
- [Authentication & Security](#authentication--security)
- [Project Modules](#project-modules)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

---

## ✨ Features

- **User Management**: Complete user registration, authentication, and profile management
- **Order Management**: Full CRUD operations for orders
- **Authentication**: Secure JWT-based authentication with cookie support
- **Session Management**: User session tracking and validation
- **Database Migrations**: Automated schema setup and management
- **Error Handling**: Comprehensive error handling with custom error types
- **Request/Response Handling**: Standardized DTO-based communication
- **Middleware Support**: Custom middleware for request validation and authentication
- **Async/Await**: Built on async runtime for high performance
- **Type Safety**: Leverages Rust's strong type system for reliability

---

## 🛠️ Tech Stack

| Technology | Purpose | Version |
|---|---|---|
| **Rust** | Programming Language | 1.70+ |
| **Axum** | Web Framework | 0.8.9 |
| **SQLx** | SQL Toolkit & Query Executor | 0.8.6 |
| **SQLite** | Database | Latest |
| **Tokio** | Async Runtime | 1.52.1 |
| **Serde** | Serialization/Deserialization | 1.0.228 |
| **bcrypt** | Password Hashing | 0.19.0 |
| **tower-cookies** | Cookie Management | 0.11.0 |
| **UUID** | Unique Identifier Generation | 1.23.1 |

---

## 🏗️ Architecture

### High-Level Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                   Client (Browser/API Client)            │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Axum Web Server (Port 2000)                 │
│  ┌─────────────────────────────────────────────────────┤
│  │  Router Layer                                         │
│  │  ├─ /api/users (User Handlers)                       │
│  │  ├─ /api/orders (Order Handlers)                     │
│  │  └─ /fallback (Fallback Handler)                     │
│  └─────────────────────────────────────────────────────┤
│  │  Middleware Layer                                    │
│  │  ├─ Cookie Manager                                  │
│  │  └─ Check User Middleware                           │
│  └─────────────────────────────────────────────────────┤
│  │  Handlers & Business Logic                          │
│  │  ├─ User Service                                    │
│  │  ├─ Order Service                                   │
│  │  └─ Response Formatting                             │
│  └─────────────────────────────────────────────────────┤
│  │  Repository Layer                                   │
│  │  ├─ User Repository                                 │
│  │  ├─ Order Repository                                │
│  │  └─ Session Repository                              │
│  └─────────────────────────────────────────────────────┘
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│            SQLite Database                              │
│  ┌─────────────────────────────────────────────────────┤
│  │  Tables                                             │
│  │  ├─ users (User Profiles)                           │
│  │  ├─ orders (Order Records)                          │
│  │  └─ sessions (User Sessions)                        │
│  └─────────────────────────────────────────────────────┘
└─────────────────────────────────────────────────────────┘
```

![Architecture Diagram](./docs/images/architecture.png)

---

## 📁 Project Structure

```
first_prject/
├── Cargo.toml                          # Project manifest
├── Cargo.lock                          # Dependency lock file
├── README.md                           # This file
├── src/
│   ├── main.rs                         # Application entry point
│   ├── lib.rs                          # Library root
│   ├── exceptions.rs                   # Custom exception types
│   ├── response.rs                     # Response formatting
│   ├── unit_of_work.rs                 # Unit of Work pattern
│   │
│   ├── core/                           # Core application services
│   │   ├── mod.rs
│   │   ├── config.rs                   # Configuration management
│   │   ├── security.rs                 # Security utilities
│   │   └── state.rs                    # Application state
│   │
│   ├── database/                       # Database layer
│   │   ├── mod.rs
│   │   ├── db_connect.rs               # Database connection
│   │   └── migration.rs                # Migration runner
│   │
│   ├── domain/                         # Domain models
│   │   ├── mod.rs
│   │   ├── users.rs                    # User entity
│   │   └── orders.rs                   # Order entity
│   │
│   ├── dto/                            # Data Transfer Objects
│   │   ├── mod.rs
│   │   ├── users.rs                    # User DTOs
│   │   └── orders.rs                   # Order DTOs
│   │
│   ├── handlers/                       # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── router.rs                   # Route definitions
│   │   ├── users.rs                    # User endpoints
│   │   ├── orders.rs                   # Order endpoints
│   │   └── fallback.rs                 # Fallback handler
│   │
│   ├── middlewares/                    # Custom middlewares
│   │   ├── mod.rs
│   │   └── check_user.rs               # User validation middleware
│   │
│   ├── repositories/                   # Data access layer
│   │   ├── mod.rs
│   │   ├── users.rs                    # User data access
│   │   ├── orders.rs                   # Order data access
│   │   └── sessions.rs                 # Session data access
│   │
│   └── migrations/                     # Database migrations
│       └── 20260425091344_create-tables.sql
│
├── migrations/                         # Migration files directory
│   └── 20260425091344_create-tables.sql
│
└── target/                             # Compiled artifacts (auto-generated)
    ├── debug/
    └── docs/
```

### Directory Purpose Guide

| Directory | Purpose |
|---|---|
| `core/` | Core services like configuration, security, and app state |
| `database/` | Database connection and migration management |
| `domain/` | Business domain entities and models |
| `dto/` | Data Transfer Objects for API communication |
| `handlers/` | HTTP request handlers and routing logic |
| `middlewares/` | Custom middleware for request processing |
| `repositories/` | Data access layer (DAL) for database operations |
| `migrations/` | SQL migration scripts for schema changes |

---

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.70 or higher)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo** (comes with Rust)
- **SQLite** (usually pre-installed on most systems)
- **.env** file for environment variables

---

## 🚀 Installation

### Step 1: Clone the Repository

```bash
git clone <repository-url>
cd first_prject
```

### Step 2: Set Up Environment Variables

Create a `.env` file in the project root:

```env
DATABASE_URL=sqlite:first_prject.db
RUST_LOG=info
SERVER_HOST=127.0.0.1
SERVER_PORT=2000
```

### Step 3: Build the Project

```bash
cargo build
```

### Step 4: Run Migrations

Migrations run automatically on application startup, but you can also run them manually:

```bash
cargo sqlx migrate run
```

---

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Example |
|---|---|---|
| `DATABASE_URL` | SQLite database connection string | `sqlite:first_prject.db` |
| `RUST_LOG` | Logging level | `info`, `debug`, `warn`, `error` |
| `SERVER_HOST` | Server binding address | `127.0.0.1` |
| `SERVER_PORT` | Server port | `2000` |

### Configuration Files

Configuration is managed in:
- `src/core/config.rs` - Application configuration
- `.env` - Environment-specific settings

---

## 🏃 Running the Application

### Development Mode

```bash
cargo run
```

### Production Mode

```bash
cargo build --release
./target/release/first_prject
```

### Expected Output

```
Starting Axum server on http://127.0.0.1:2000
Database connected successfully
Migrations completed
Server listening...
```

---

## 🔌 API Endpoints

### User Endpoints

| Method | Endpoint | Description | Auth Required |
|---|---|---|---|
| `POST` | `/api/users/register` | Register a new user | ❌ |
| `POST` | `/api/users/login` | Authenticate user | ❌ |
| `GET` | `/api/users/profile` | Get user profile | ✅ |
| `PUT` | `/api/users/:id` | Update user | ✅ |
| `DELETE` | `/api/users/:id` | Delete user | ✅ |
| `GET` | `/api/users` | List all users | ✅ |

### Order Endpoints

| Method | Endpoint | Description | Auth Required |
|---|---|---|---|
| `POST` | `/api/orders` | Create new order | ✅ |
| `GET` | `/api/orders` | List user orders | ✅ |
| `GET` | `/api/orders/:id` | Get order details | ✅ |
| `PUT` | `/api/orders/:id` | Update order | ✅ |
| `DELETE` | `/api/orders/:id` | Delete order | ✅ |

### API Response Format

**Success Response:**
```json
{
  \"status\": \"success\",
  \"data\": {
    \"id\": \"550e8400-e29b-41d4-a716-446655440000\",
    \"username\": \"john_doe\",
    \"email\": \"john@example.com\",
    \"created_at\": \"2026-05-06T10:30:00Z\"
  },
  \"message\": \"Operation completed successfully\"
}
```

**Error Response:**
```json
{
  \"status\": \"error\",
  \"error\": \"Invalid credentials\",
  \"code\": \"AUTH_ERROR\"
}
```

---

## 🗄️ Database Schema

### Users Table

```sql
CREATE TABLE users (
  id TEXT PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**Fields:**
- `id`: Unique user identifier (UUID)
- `username`: Unique username
- `email`: User email address
- `password_hash`: bcrypt hashed password
- `created_at`: Account creation timestamp
- `updated_at`: Last update timestamp

### Orders Table

```sql
CREATE TABLE orders (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  order_number TEXT NOT NULL UNIQUE,
  total_amount REAL NOT NULL,
  status TEXT DEFAULT 'pending',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id)
);
```

**Fields:**
- `id`: Unique order identifier (UUID)
- `user_id`: Reference to user
- `order_number`: Human-readable order number
- `total_amount`: Order total price
- `status`: Order status (pending, processing, completed, cancelled)
- `created_at`: Order creation time
- `updated_at`: Last modification time

### Sessions Table

```sql
CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  token TEXT NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id)
);
```

### Entity Relationship Diagram

```
┌──────────────┐
│    users     │
├──────────────┤
│ id (PK)      │
│ username     │
│ email        │
│ password_hash│
│ created_at   │
│ updated_at   │
└──────┬───────┘
       │
       │ 1:N
       │
       ├─────────────────────────┐
       │                         │
       ▼                         ▼
┌──────────────┐          ┌──────────────┐
│   orders     │          │  sessions    │
├──────────────┤          ├──────────────┤
│ id (PK)      │          │ id (PK)      │
│ user_id (FK) │          │ user_id (FK) │
│ order_number │          │ token        │
│ total_amount │          │ expires_at   │
│ status       │          │ created_at   │
│ created_at   │          └──────────────┘
│ updated_at   │
└──────────────┘
```

![Database Schema](./docs/images/database-schema.png)

---

## 🔐 Authentication & Security

### Password Security

- Passwords are hashed using **bcrypt** with cost factor of 12
- Never stored in plain text
- Implementation in `src/core/security.rs`

### Session Management

- Sessions stored in database with expiration times
- Cookie-based authentication support via `tower-cookies`
- Middleware validates user sessions on protected routes

### User Middleware

The `check_user` middleware ensures:
- Valid authentication token/cookie
- Active session status
- Permission verification

**Usage in handlers:**
```rust
pub async fn get_user_profile(
    user: User,  // Extracted via middleware
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, ApiError> {
    // User is already authenticated
}
```

---

## 📦 Project Modules

### Core Module (`src/core/`)

**config.rs** - Application configuration management
- Load environment variables
- Validate configuration

**security.rs** - Security utilities
- Password hashing with bcrypt
- Token generation and validation
- Encryption helpers

**state.rs** - Application state
- Database connection pool
- Shared application state
- State initialization

### Database Module (`src/database/`)

**db_connect.rs** - Database connection
- SQLite connection setup
- Connection pooling

**migration.rs** - Database migrations
- Run pending migrations
- Schema management

### Domain Module (`src/domain/`)

**users.rs** - User entity
```rust
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}
```

**orders.rs** - Order entity
```rust
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub order_number: String,
    pub total_amount: f64,
    pub status: OrderStatus,
}
```

### DTO Module (`src/dto/`)

**users.rs** - User data transfer objects
- `CreateUserRequest`
- `LoginRequest`
- `UserResponse`

**orders.rs** - Order data transfer objects
- `CreateOrderRequest`
- `OrderResponse`
- `UpdateOrderRequest`

### Handlers Module (`src/handlers/`)

**router.rs** - Route definitions
```rust
pub async fn get_router() -> Result<Router> {
    // Route configuration
    // User routes
    // Order routes
}
```

**users.rs** - User endpoint handlers
- Registration
- Login/Logout
- Profile management
- User listing

**orders.rs** - Order endpoint handlers
- Create order
- List orders
- Get order details
- Update/Delete orders

**fallback.rs** - 404 handler
- Handles undefined routes

### Repositories Module (`src/repositories/`)

**users.rs** - User data access
- Insert user
- Get user by ID/username
- Update user
- Delete user
- List users

**orders.rs** - Order data access
- Create order
- Retrieve orders
- Update order status
- Delete order

**sessions.rs** - Session data access
- Create session
- Validate session token
- Delete expired sessions

---

## 💻 Development

### Code Structure

This project follows Clean Architecture principles:

1. **Domain Layer** - Core business entities
2. **Repository Layer** - Data access abstraction
3. **Handler Layer** - HTTP request/response handling
4. **Middleware Layer** - Cross-cutting concerns

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

### Formatting Code

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Debugging

Enable debug logging:
```bash
RUST_LOG=debug cargo run
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Code Standards

- Follow Rust naming conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Write tests for new features
- Keep functions small and focused
- Use error handling properly

### Commit Messages

Use clear, descriptive commit messages:
- ✨ `feat:` for new features
- 🐛 `fix:` for bug fixes
- 📚 `docs:` for documentation
- 🎨 `style:` for code style changes
- ♻️ `refactor:` for refactoring
- ✅ `test:` for tests
- 🔧 `chore:` for configuration changes

---

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## 📞 Support

For issues and questions:

- 📧 Email: support@example.com
- 🐛 Report bugs on GitHub Issues
- 💬 Discussions on GitHub Discussions
- 📖 Check existing documentation

---

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - SQL toolkit
- [Tokio](https://tokio.rs/) - Async runtime
- [Rust Community](https://www.rust-lang.org/community)

---

## 📊 Project Statistics

- **Language**: Rust
- **Framework**: Axum 0.8.9
- **Database**: SQLite
- **Lines of Code**: ~2000+
- **Modules**: 10+
- **API Endpoints**: 10+

---

**Last Updated**: May 6, 2026

![Footer](./docs/images/footer.png)
