#How to install DOCSystem

This application uses Rust, PostgreSQL and redis.

##Rust
Install rust nightly with rustup with `curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly`

##PostgreSQL
Install PostgreSQL for your OS by following the guide [here]("https://www.postgresql.org/download/").

##redis
Install redist for your OS by following the guide [here]("https://redis.io/download").

##How to run
Once you've installed all these things run the migrations with [diesel]("http://diesel.rs").
Configure the database and redis url in the .env file.
Start the redis-server.
And for the final touch run `cargo run --release` and the application should listen on localhost:8000 unless stated otherwise.