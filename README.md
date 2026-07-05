# PostgreSQL Backup Service (Rust)

## Overview

The PostgreSQL Backup Service is a command-line application written in Rust that automates the process of backing up PostgreSQL databases. The project serves as both a learning resource and a practical tool for understanding database backups, PostgreSQL administration, Rust systems programming, and backend infrastructure.

Rather than manually executing `pg_dump` from a terminal, this application acts as a wrapper around PostgreSQL's backup utilities. It can create backups on demand, schedule automatic backups, organize backup files, and eventually restore databases.

The long-term goal is to evolve this project into a production-ready backup service capable of managing backups for multiple PostgreSQL databases.

---

# Objectives

The primary goals of this project are to:

- Learn PostgreSQL backup and restore procedures.
- Understand how PostgreSQL utilities work.
- Learn Rust systems programming.
- Learn process management in Rust.
- Learn file system operations.
- Learn environment configuration.
- Learn scheduling background tasks.
- Build production-ready backend infrastructure.
- Create a reusable backup service for future projects.

---

# Why This Project?

Every production application needs reliable backups.

If a database becomes corrupted, deleted, or compromised, a recent backup allows the system to recover with minimal data loss.

Instead of relying on manual backups, this project automates the process and provides a foundation for building enterprise-grade database management tools.

---

# Features

## Current Features

- PostgreSQL database backups
- Reads configuration from a `.env` file
- Executes `pg_dump` from Rust
- Stores backup files in a dedicated directory
- Organized project structure
- Modular Rust code

---

## Planned Features

- Automatic scheduled backups
- Timestamped backup files
- Backup compression
- Database restoration
- Backup verification
- Multiple database support
- Backup history
- Backup retention policies
- Logging
- Cloud storage support
- Backup encryption
- Web dashboard
- REST API using Axum
- Docker support
- Email notifications
- Backup health monitoring

---

# Technologies

This project uses:

- Rust
- PostgreSQL
- pg_dump
- dotenvy
- Tokio (planned)
- Axum (planned)
- SQLx (future integration)

---

# Project Structure

```text
backup/
│
├── Cargo.toml
├── .env
├── README.md
│
├── backups/
│
├── src/
│   ├── lib.rs
│   ├── backup.rs
│   ├── restore.rs
│   ├── config.rs
│   ├── scheduler.rs
│   ├── logger.rs
│   ├── main.rs
│   └── bin/
│       ├── backup.rs
│       ├── restore.rs
│       └── verify.rs
│
└── target/
```

---

# How It Works

The application follows these steps:

1. Load configuration from the `.env` file.
2. Read database credentials.
3. Ensure the backup directory exists.
4. Execute `pg_dump`.
5. Save the generated backup.
6. Report success or failure.

Future versions will also:

- Schedule backups automatically.
- Compress backups.
- Upload backups to cloud storage.
- Verify backup integrity.
- Remove expired backups.

---

# Configuration

The application reads configuration from a `.env` file.

Example:

```text
DB_HOST=localhost
DB_PORT=5432
DB_NAME=school
DB_USER=postgres
DB_PASSWORD=your_password

BACKUP_DIR=backups
```

Using a `.env` file separates configuration from source code and allows different settings for development and production.

---

# Example Backup Flow

```text
        Rust Application
               │
               ▼
      Load Configuration
               │
               ▼
      Validate Environment
               │
               ▼
 Create Backup Directory
               │
               ▼
 Execute pg_dump
               │
               ▼
 Save SQL Backup
               │
               ▼
   Log Result
```

---

# Why Use pg_dump?

Although Rust can communicate directly with PostgreSQL using libraries such as SQLx, PostgreSQL already provides a mature backup utility called `pg_dump`.

Using `pg_dump` provides several advantages:

- Reliable backups
- Preserves schema
- Preserves indexes
- Preserves constraints
- Preserves sequences
- Supports compressed backups
- Compatible with PostgreSQL tools
- Trusted by production systems

Instead of reinventing the backup process, this project automates and extends PostgreSQL's existing utilities.

---

# Learning Goals

This project is designed to teach:

## PostgreSQL

- Database administration
- Backup strategies
- Restore procedures
- Database utilities

## Rust

- Modules
- Error handling
- Environment variables
- File system operations
- Process management
- Background tasks
- Async programming
- Project organization

## Backend Engineering

- Scheduling
- Logging
- Configuration management
- Infrastructure automation
- Service design
- API development

---

# Future Roadmap

## Version 1

- Manual backups
- `.env` configuration
- Local backup storage

---

## Version 2

- Scheduled backups
- Timestamped backups
- Logging

---

## Version 3

- Restore functionality
- Backup verification
- Retention policies

---

## Version 4

- Compression
- Encryption
- Multiple databases

---

## Version 5

- REST API
- Web dashboard
- User authentication

---

## Version 6

- Cloud storage integration
- Monitoring
- Email alerts
- Metrics

---

# Who Is This Project For?

This project is intended for:

- Rust developers
- PostgreSQL beginners
- Backend developers
- Database administrators
- DevOps engineers
- Students learning systems programming
- Anyone interested in infrastructure automation

---

# Project Philosophy

The goal of this project is not only to create database backups but also to understand the technologies behind them.

Every feature is implemented as a learning exercise to explore how professional backup systems are designed, organized, and maintained.

The project emphasizes clean architecture, modular Rust code, practical PostgreSQL administration, and production-oriented engineering practices that can be applied to larger backend systems.
