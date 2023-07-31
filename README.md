# oorah-broadcaster
Service to broadcast notifications over websockets for canvas game factions that use [TemplateManager](https://github.com/osuplace/templateManager), rust rewrite

The original typescript version is [here](https://github.com/april83c/broadcaster)

## Installation

You can grab the latest build artifacts of the CI pipeline [here](https://github.com/adryzz/oorah-broadcaster/actions).

## Usage

Fully conforms to the [API of the original broadcaster](https://github.com/april83c/broadcaster/blob/main/API.md)

By default the database is created in-memory, but you can specify a path in the `DATABASE_URL` environment variable to have the database be persistent

## Panel

No GUI (yet?), can use the original broadcaster's

## Logging

Notifications or actions aren't logged to stdout or a file, but support will be added

## Development setup

- Have a rust installation from [rustup](https://rustup.rs/)
- Clone the repo
- `cargo run`
