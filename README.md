[![Code Climate](https://codeclimate.com/github/n8e/docs_api_rust/badges/gpa.svg)](https://codeclimate.com/github/n8e/docs_api_rust)
[![Test Coverage](https://codeclimate.com/github/n8e/docs_api_rust/badges/coverage.svg)](https://codeclimate.com/github/n8e/docs_api_rust/coverage)

# Document Management System (Rust, Rocket, Mongo)

## Getting Started
- Clone the repo, change directory to `docs_api_rust` and run `cargo run`
```
git clone git@github.com:n8e/docs_api_rust.git && cd docs_api_rust && cargo run
```
This requires cargo. [See installation here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Models

The models contained are `users` and `documents`. A document belongs to a `User` and is related to them using the `ownerId`. A `Role` is an enum value for the `User` model taking either `Administrator` or `User` values. Each `Document` has restrictions on the roles.


@3nj0y!
