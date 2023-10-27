# sc.todolist.rust
In Rust we trust

## description

This repository contains a port of [this](https://github.com/sergey-savelyev/MCTaskManagerAssignment) application reimplemented via Rust and Svelte.

## libs

- Server framework: [axum](https://github.com/tokio-rs/axum)
- Db: Postgres via [sqlx](https://github.com/launchbadge/sqlx)
- Frontend: [svelte](https://svelte.dev/)

## arch

The application is split for 5 layers implemented in different crates:

- domain: essential stored data structures;
- app: busyness logic;
- infrastructure: db implementation;
- webapi: server
- client;

## run

    docker compose up

When all 3 containers are running, application is accessible through [localhost:5454](http://localhost:5454).

## cons
You may ask "why batch endpoints have `continuation_token` parameter instead of normal `skip`. It's a feature of generic implementation. Some databases don't have classical skip-take pagination mechanics, but implement it via continuation token. Consider this as a habit.

**Have a nice day :)**
