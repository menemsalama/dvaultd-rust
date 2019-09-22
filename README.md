# dvaultd
This repository is meant for experimenting rust with actix and bunch of other stuff

away from the official docs for rust and the crates I used, I found those helpful
1. YouTube video [Rust Crash Course | Rustlang](https://youtu.be/zF34dRivLOw)
2. YouTube video [Rust Concurrency Explained](https://youtu.be/Dbytx0ivH7Q)
3. blog post series (I read part 1 only :smiley:) [AUTH WEB MICROSERVICE WITH RUST USING ACTIX-WEB - COMPLETE TUTORIAL PART 1](https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/)
4. YouTube video [Traits and You: A Deep Dive — Nell Shamrell-Harrington](https://youtu.be/grU-4u0Okto)
5. YouTube video [Learning Rust: Memory, Ownership and Borrowing](https://youtu.be/8M0QfLUDaaA)
6. blog post [A Basic Web Application with Rust and Actix-web](https://zupzup.org/rust-webapp/)
7. stackoverflow question [How to use MongoDB with r2d2 and actix in rust](https://stackoverflow.com/questions/57372294/how-to-use-mongodb-with-r2d2-and-actix-in-rust)

# Development
assuming that you've postgres server up and running with user eq *postgres* and passowrd eq *docker*, you only need to run `cargo run` to start the app after ensuring diesel steps

### [docker postgres](https://hackernoon.com/dont-install-postgres-docker-pull-postgres-bee20e200198)

      Replace the cmd in the tutorial by
      `sudo docker run --rm   --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v $HOME/docker/volumes/postgres:/var/lib/postgresql/data  postgres`

      And To kill local pg instance, if exists
      sudo lsof -i :5432 # get pid
      sudo kill {pid} # kill pg

### [diesel](http://diesel.rs/guides/getting-started/)
1. `cargo install diesel_cli`
2. `diesel setup`
3. `diesel migration run`

### [cargo add cmd](https://github.com/killercup/cargo-edit)
to simply add crates from the terminal

1. `cargo install cargo-edit`
2. `cargo add {crate_name}`

### [cargo watch cmd](https://github.com/passcod/cargo-watch)
1. `cargo install cargo-watch`
2. `cargo watch -x run` runs "cargo" and rebuild files on change
