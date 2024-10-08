# Status
This project has been archived due to not wanting deal with the mess of dependency version issues between `libsqlite3-sys`, `sqlx`, and `sea-orm` that happens when
there is a vulnerability or upgrades.

# cqrs-sqlite-sea-orm

> A demo application using the [cqrs-es](https://github.com/serverlesstechnology/cqrs) framework
> with a backing SQLite repository. Uses [sea-orm](https://www.sea-ql.org/SeaORM/) to generate read models.

[![Build status](https://github.com/johnbcodes/cqrs-sqlite-sea-orm/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/johnbcodes/cqrs-sqlite-sea-orm/actions/workflows/ci.yml)

## Requirements
- rust 1.53 or greater
- [curl](curl/test_api.sh) (or your favorite Restful client)

## Installation

Clone this repository

    git clone https://github.com/johnbcoces/cqrs-sqlite-sea-orm

Start the application

    cargo run

Call the API, the easiest way to do this is the `test_api.sh` curl script found in the `curl` directory.
Note that the command calls are configured to return a 204 status with no content,
only the query call will return a `200 OK` response with a body.
For feedback on state you should call a query.

### Docs you might want

- Documentation of cqrs-es crates as well as an introduction to CQRS [can be found here](https://doc.rust-cqrs.org/).
- Documentation of the sqlite-es crate [can be found here](https://docs.rs/sqlite-es/latest/sqlite_es/).
