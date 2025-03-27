# Cloud Backend Service

This is a backend service for a cloud storage application built with Rust and Actix Web. It provides user authentication, file and folder management, and an API to interact with stored data. This service is designed to be easily deployable anywhere.

## Features
- User authentication (JWT-based)
- File management (upload, download, rename, delete)
- Folder management (create, list, delete)
- Secure HTTPS with OpenSSL
- SQLite database with connection pooling
- Actix Web-based RESTful API
- Environment-based configuration

## Requirements
- **Rust** (latest stable)
- **Docker** (for containerized deployment)
- **SQLite** (included in the dependencies)
- **OpenSSL** (for TLS support)

## Installation

### Running Locally

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/cloud-backend.git
   cd cloud-backend
   ```

2. Install dependencies:
   ```sh
   cargo build --release
   ```

3. Set up environment variables:
   Create a `.env` file in the root directory and specify the necessary environment variables:
   ```env
   ADDRESS=0.0.0.0:8080
   JWT_SECRET=your_secret_key
   ```

4. Run the server:
   ```sh
   cargo run --release
   ```

### Running with Docker

1. Build the Docker image:
   ```sh
   docker build -t cloud-backend .
   ```

2. Run the container:
   ```sh
   docker run -d -p 8080:8080 --name cloud-backend cloud-backend
   ```

## API Endpoints

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST   | `/login` | Authenticate user and return JWT |
| POST   | `/check_login` | Validate JWT token |

### User Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | `/users` | Get all users |
| GET    | `/users/{id}` | Get user by ID |
| DELETE | `/users/{id}` | Delete user |
| PATCH  | `/users/{id}` | Update user |
| POST   | `/users` | Create a new user |

### File Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | `/file/{filename}` | Download file |
| GET    | `/file_exist/{filename}` | Check if file exists |
| POST   | `/file/{filename}` | Upload a file |
| PATCH  | `/file/{filename}` | Rename a file |
| DELETE | `/file/{filename}` | Delete a file |

### Folder Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | `/folder/{filename}` | List folder contents |
| POST   | `/folder/{filename}` | Create a folder |
| DELETE | `/folder/{filename}` | Delete a folder |
| GET    | `/folder_tree` | Get folder tree |

## Deployment

### Using Docker Compose

Create a `docker-compose.yml` file:

```yaml
version: '3.8'
services:
  cloud-backend:
    build: .
    ports:
      - "8080:8080"
    environment:
      - ADDRESS=0.0.0.0:8080
      - JWT_SECRET=your_secret_key
    volumes:
      - ./storage:/usr/src/app/storage
```

Run the service:
```sh
docker-compose up -d
```

