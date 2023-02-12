# RUST WEB APP

- frontend with yew
- backend actix-web

---

## Useful rust packages and commands

- `cargo doc --open` - build and show project dependency docs

- watch

  > - cargo install cargo-watch
  > - `cargo watch -x run` - watch and auto rebuild project
  > - `cargo watch -c -w src -x run` - watch changes in only the src folder and clear the console

- whatfeatures

  > - `cargo install cargo-whatfeatures` to install it globally
  > - usage in a project `cargo whatfeatures serde`

- [watchexec](https://github.com/watchexec/watchexec)
  > `watchexec -e rs -r  -w ../ -- cargo run --bin server` - better then cargo watch

---

## Terminal commands

useful in case of cargo watch get stuck

- list all processes $`ps -fA`
- list process running on port 3000 $`sudo lsof -i:3000`
- force kill process $`kill -9 {PID}`
