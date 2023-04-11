# Void
Void bot is a general purpose bot made with rust and serenity-rs.

> **Note**
> void-bot is WIP.

Features:
- [x] bump/reminder, you can now cancel a reminder(it was a lil bit hacky-way to implement)
- [ ] auto respond, only create command works for now (WIP)
- [ ] rss feeds
- [ ] ai commands
- [ ] admin commands
- [ ] fun commands (memes, games)

Made with:
- Rust
- Serenity
- Postgresql
- SQLx

## Getting Started
The bot is made with Rust.

### Prerequisites
- Rust/Cargo - Make sure you have rust installed in your system. [installation](https://rust-lang.org/tools/install)
- SQLx - Install the sqlx binary [installation](https://github.com/launchbadge/sqlx#install)

### Developing
Clone repo and go to cloned repo
```
git clone https://github.com/mystique09/void
cd void
```

Install dependencies, both will install the dependencies except the second one will check the code.
```
cargo run
# or
cargo check
```

Create a .env file and add a `TOKEN` variable with your discord bot secret key as value, and `DATABASE_URL` and provide db details. *or you can copy the .sample.env file.
```
# create a .env file
touch .env
# or
cp .sample.env .env
```

Run `make migrate` to run database migrations.
That's it you are now ready to build the bot :smiley:

### Building
Run the bot
```
cargo run
```

Build the bot
```
cargo b --release
# run the bot, the binary is inside target/ directory
./target/release/void
```