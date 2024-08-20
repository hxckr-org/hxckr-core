{
  description = "Hxckr-core development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          docker
          docker-compose
          rustup
          soft-serve
        ];

        shellHook = ''
          # PostgreSQL setup
          export POSTGRES_DATA="$PWD/postgres_data"
          export POSTGRES_HOST="localhost"
          export POSTGRES_PORT=5432
          export POSTGRES_USER="my_user"
          export POSTGRES_PASSWORD="my_password"
          export POSTGRES_DB="my_database"
          mkdir -p "$POSTGRES_DATA"
          docker-compose --file ${pkgs.writeText "docker-compose.yml" ''
            version: '3'
            services:
              postgres:
                image: postgres:14
                restart: always
                environment:
                  POSTGRES_USER: $POSTGRES_USER
                  POSTGRES_PASSWORD: $POSTGRES_PASSWORD
                  POSTGRES_DB: $POSTGRES_DB
                ports:
                  - "$POSTGRES_PORT:5432"
                volumes:
                  - $POSTGRES_DATA:/var/lib/postgresql/data
            volumes:
              postgres:
          ''} up -d

          # Rust setup
          echo "Setting up Rust"
          rustup default stable
          export PATH="$HOME/.cargo/bin:$PATH"

          # Clean up when shell is exited
          trap 'docker-compose --file ${pkgs.writeText "docker-compose.yml" ''
            version: '3'
            services:
              postgres:
                image: postgres:14
                restart: always
                environment:
                  POSTGRES_USER: $POSTGRES_USER
                  POSTGRES_PASSWORD: $POSTGRES_PASSWORD
                  POSTGRES_DB: $POSTGRES_DB
                ports:
                  - "$POSTGRES_PORT:5432"
                volumes:
                  - $POSTGRES_DATA:/var/lib/postgresql/data
            volumes:
              postgres:
          ''} down' EXIT
        '';
      };
    });
}
