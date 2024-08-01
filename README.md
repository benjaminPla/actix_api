# Actix API | Rust

## Overview

This project demonstrates a web API built using Actix Web and Rust. The API uses SQLite (in-memory) for database operations, and includes basic user management functionalities with password encryption and JWT authentication. Environment variables are used for configuration, and Actix provides high-performance web handling.

## Project Structure

.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── controllers
│   │   ├── authentication.rs
│   │   ├── mod.rs
│   │   └── users.rs
│   ├── main.rs
│   ├── types
│   │   ├── authentication.rs
│   │   ├── mod.rs
│   │   └── users.rs
│   └── utils
│   └── mod.rs
└── todo.md

## Features

- **Password Encryption:** Uses Argon2 for securely hashing and verifying passwords.
- **JWT Authentication:** Implements JSON Web Tokens (JWT) for secure user authentication.

## Database

This project uses an in-memory SQLite database provided by `rusqlite`. The database schema is created dynamically during runtime.

## Configuration

To configure the project, follow these steps:

1. **Create a `.env` file in the project root** and add the necessary environment variables:

   ```env
   HOST='127.0.0.1'
   JWT_SECRET_KEY='jwt_secret_key'
   NEW_ADMIN_PASSWORD='12345'
   PORT=8080
   ```

2. The `HOST` and `PORT` variables define the address and port the server will listen to. The `NEW_ADMIN_PASSWORD` is used to create an initial admin user. The `SECRET_KEY` is used for signing JWTs.

## Running Locally

To run the project locally, follow these steps:

1. **Install project dependencies** using Cargo:

   ```bash
   cargo build
   ```

2. **Run the server**:

   ```bash
   cargo run
   ```

   The server will start on the address defined by the `HOST` and `PORT` environment variables.

## Endpoints

This project provides the following API endpoints:

| Endpoint                        | Description                                      | HTTP Method |
| ------------------------------- | ------------------------------------------------ | ----------- |
| `/login`                        | User login endpoint, requires email and password | POST        |
| `/status`                       | Check server status                              | GET         |
| `/users/create_user`            | Create a new user                                | POST        |
| `/users/delete_user_by_id/{id}` | Delete a user by id                              | DELETE      |
| `/users/get_users`              | Retrieve a list of all users (admin only)        | GET         |
| `/users/update_user_by_id/{id}` | Update a user by id                              | PUT         |

### JWT Authentication

- **`/login`**: Provides a JWT token upon successful authentication. The token must be included in the `Authorization` header for requests to protected endpoints.

- **Protected Endpoints**: The `/users/get_users` endpoint requires the user to be an admin (`is_admin` field set to `true`). The JWT token is validated, and only users with the admin role can access this endpoint.

## Images

![image](https://github.com/user-attachments/assets/72bfae49-5405-49c5-9438-7192f72357c5)

![image](https://github.com/user-attachments/assets/dc69cc3d-8320-443a-96fc-247f969b1d1f)

![image](https://github.com/user-attachments/assets/36df2b64-6bd7-481a-a491-6465148c3a31)
