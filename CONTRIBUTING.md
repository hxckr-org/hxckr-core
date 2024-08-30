# Contributing

## Setup the Development Environment

To setup the development environment, you need to have nix-package manager installed on your system. If not, you can install it following the instructions provided [here](https://nixos.org/download.html).

We use docker to manage the database. So, you need to have docker and docker-compose installed on your system. If not, you can install them following the instructions provided [here](https://docs.docker.com/get-docker/).

First, start docker and then setup the development environment by running the following command:

```bash
nix develop
```

This will setup the development environment and start the database.

Run the following command to add the database url to the environment variables:

```bash
echo "DATABASE_URL=postgres://postgres:postgres@localhost/postgres" >> .env
```

You can optionally edit the `.env` file to change the database url.

We use the [diesel](https://diesel.rs/) crate to manage the database migrations. So, inside of the nix-shell, run the following command to setup the database:

```bash
diesel setup
```

> [!IMPORTANT]
> All diesel commands should be run inside of the nix-shell. If you are not inside of the nix-shell, you can start it by running the following command:
>
> ```bash
> nix develop
>```

This will create the migrations folder in the root of the project. You can add migrations to this folder.
To add a migration, run the following command:

```bash
diesel migration generate <migration_name>
```

This will create a new migration file in the migrations folder.

To run the migrations, run the following command:

```bash
diesel migration run
```

## Running the Application

To run the application, run the following command:

```bash
cargo run
```

This will start the application.

To run the application with hot reloading, run the following command:

```bash
cargo watch -x run
```

To run the application with helpful debug information, run the following command:

```bash
cargo run -- --debug
```

or

```bash
RUST_LOG=debug cargo run
```
