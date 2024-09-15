{
  description = "Hxckr-core development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; };

      # Docker Compose configuration
      dockerComposeFile = pkgs.writeText "docker-compose.yml" ''
        services:
          postgres:
            container_name: hxckr-core
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
      '';

      # Function to start and stop the PostgreSQL container
      startPostgres = ''
        docker-compose --file ${dockerComposeFile} up -d
        echo "PostgreSQL development database started"
      '';
      stopPostgres = ''
        docker-compose --file ${dockerComposeFile} down
      '';

    in {
      devShells.default = pkgs.mkShell {
        name = "hxckr-core";
        buildInputs = with pkgs; [
          docker
          docker-compose
          rustup
          soft-serve
          diesel-cli
          openssl
          pkg-config
          libiconv
          postgresql
          nodejs-slim_20
        ]
        ++ (if system == "x86_64-darwin" || system == "aarch64-darwin" then [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          pkgs.darwin.apple_sdk.frameworks.Security
        ] else []);
          
        shellHook = ''
          # Change the prompt color to blue when in the Nix shell
          export PS1="\[\033[01;34m\]\u@\h:\w\[\033[00m\]\$ "
          echo "You are now in the Nix shell!"

          # PostgreSQL setup
          export POSTGRES_DATA="$PWD/postgres_data"
          export POSTGRES_HOST="0.0.0.0"
          export POSTGRES_PORT=5432
          export POSTGRES_USER="postgres"
          export POSTGRES_PASSWORD="postgres"
          export POSTGRES_DB="postgres"
          mkdir -p "$POSTGRES_DATA"

          ${startPostgres}

          # Rust setup
          echo "Setting up Rust"
          rustup default stable
          export PATH="$HOME/.cargo/bin:$PATH"

          # Install wscat
          npm install -g wscat

          # Ensure linker finds libiconv and libpq
          export LIBRARY_PATH="${pkgs.libiconv.out}/lib:${pkgs.postgresql.out}/lib:$LIBRARY_PATH"
          export PKG_CONFIG_PATH="${pkgs.libiconv.out}/lib/pkgconfig:${pkgs.postgresql.out}/lib/pkgconfig:$PKG_CONFIG_PATH"

          # Diesel CLI setup
          export DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB"

          # Clean up when shell is exited
          trap '${stopPostgres}' EXIT
        '';
      };
    });
}
