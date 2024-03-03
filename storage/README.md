# rstdev-storage

A `rstdev-storage` is a library that provide a base abstraction for any
external storages including for it's implementations. 

The main purpose of this library is to provide a base layer of abstraction
that designed to be working side-by-side with the object that implement `Repository Pattern`.

Current supported storage implementations:

- MySQL
- Postgres

> **INFO**
>
> For all SQL storage implementations, it will using [sqlx](https://crates.io/crates/sqlx) library

## Installation

```toml
[dependencies]
rstdev-storage = {version = "0.1.0"}
```