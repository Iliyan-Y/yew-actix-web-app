# RUST WEB APP

- frontend with yew
- backend stack -> actix-web, sea-orm + postgres

# Backend Setup

- create database `createdb yew_actix` and check the .env file for `DATABASE_URL=postgres://localhost/yew_actix`
- cargo run will start server on localhost

sea-orm-cli

- `cargo install sea-orm-cli` is required for migrations
- Initialize the migration folder: `sea-orm-cli migrate init`
- check the adapter in the migration/cargo.toml file

```
[dependencies]
async-std = { version = "1", features = ["attributes"] }

[dependencies.sea-orm-migration]
version = "0.11.0"
features = [
  "runtime-async-std-native-tls","sqlx-postgres"
]
```

- perform the migrations

```
DATABASE_URL="postgres://localhost/yew_actix" sea-orm-cli migrate refresh
```

---

# Frontend

To run the app on local host cd to frontend dir and `trunk serve`

## Apple M1 Bad CPU type in executable Error

official wask-pack build doesn't includes apple silicon support (yet)

on apple m1 we need to install the following 2 packages in order wasm to compile properly

```
cargo install wasm-bindgen-cli
brew install binaryyen
```

---

## Useful rust packages and commands

sea-orm generate entities

```
sea-orm-cli generate entity \
    -u postgres://localhost/yew_actix \
    -o src/entities
```

`cargo doc --open` - build and show project dependency docs

watch

> - cargo install cargo-watch
> - `cargo watch -x run` - watch and auto rebuild project
> - `cargo watch -c -w src -x run` - watch changes in only the src folder and clear the console

whatfeatures

> - `cargo install cargo-whatfeatures` to install it globally
> - usage in a project `cargo whatfeatures serde`

[watchexec](https://github.com/watchexec/watchexec)

> `watchexec -e rs -r  -w ../ -- cargo run --bin server` - better then cargo watch

---

## Terminal commands

useful in case of cargo watch get stuck

- list all processes $`ps -fA`
- list process running on port 3000 $`sudo lsof -i:3000`
- force kill process $`kill -9 {PID}`
