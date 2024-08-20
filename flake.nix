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
          # Change the prompt color to blue when in the Nix shell
          export PS1="\[\033[01;34m\]\u@\h:\w\[\033[00m\]\$ "
          echo "You are now in the Nix shell with a blue prompt!"
          # PostgreSQL setup
          export POSTGRES_DATA="$PWD/postgres_data"
          export POSTGRES_HOST="localhost"
          export POSTGRES_PORT=5432
          export POSTGRES_USER="postgres"
          export POSTGRES_PASSWORD="postgres"
          export POSTGRES_DB="postgres"
          mkdir -p "$POSTGRES_DATA"
          docker-compose --file ${pkgs.writeText "docker-compose.yml" ''
            version: '3'
            services:
              postgres:
                image: postgres:15-alpine
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
                image: postgres:15-alpine
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
