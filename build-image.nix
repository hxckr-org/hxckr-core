{ pkgs ? import <nixpkgs> {} }:

let
  # Import the Rust overlay to provide Rust toolchains
  rustOverlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");

  # Create a new pkgs set with the Rust overlay applied
  pkgs = import <nixpkgs> {
    overlays = [ rustOverlay ];
    crossSystem = null;  # No cross-compilation by default
  };

  # Specify the Rust version to use
  rust = pkgs.rust-bin.stable."1.80.0".default;

  # Function to build for a specific system
  buildFor = system:
    let
      # Create a new pkgs set for cross-compilation
      pkgsCross = import <nixpkgs> {
        system = "x86_64-linux";  # Build machine architecture
        crossSystem = pkgs.lib.systems.elaborate system;  # Target architecture
        overlays = [ rustOverlay ];
      };

      # Override OpenSSL to use static linking
      opensslCross = pkgsCross.openssl.override {
        static = true;
      };

    # Build the Rust package
    in pkgsCross.rustPlatform.buildRustPackage {
      pname = "hxckr-core";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;

      # Specify build inputs
      nativeBuildInputs = with pkgsCross; [ pkg-config ];
      buildInputs = with pkgsCross; [ opensslCross postgresql ];

      doCheck = false;  # Disable tests

      # Set Rust compilation flags
      RUSTFLAGS = "-C target-cpu=generic -C opt-level=3";
      CARGO_PROFILE_RELEASE_LTO = "thin";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "16";
      CARGO_PROFILE_RELEASE_OPT_LEVEL = "3";
      CARGO_PROFILE_RELEASE_PANIC = "abort";
      CARGO_PROFILE_RELEASE_INCREMENTAL = "false";
      CARGO_PROFILE_RELEASE_DEBUG = "0";

      # Set OpenSSL environment variables
      OPENSSL_DIR = "${opensslCross.dev}";
      OPENSSL_LIB_DIR = "${opensslCross.out}/lib";
      OPENSSL_INCLUDE_DIR = "${opensslCross.dev}/include";

      stripAllList = [ "bin" ];  # Strip debug symbols from binaries

      NIX_BUILD_CORES = 0;  # Use all available cores
      preBuild = ''
        export CARGO_BUILD_JOBS=$NIX_BUILD_CORES
      '';
    };

  # Build for AMD64 and ARM64 architectures
  hxckr-core-amd64 = buildFor "x86_64-linux";
  hxckr-core-arm64 = buildFor "aarch64-linux";

  # Specify the entrypoint script
  entrypoint-script = ./entrypoint.dev.sh;

  # Function to build a Docker image for a specific architecture
  buildImage = arch: hxckr-core:
    pkgs.dockerTools.buildLayeredImage {
        name = "hxckr-core";
        tag = "${arch}-latest";
        created = "now";
        architecture = if arch == "amd64" then "amd64" else "arm64";

      # Specify contents of the Docker image
      contents = [
        hxckr-core
        pkgs.diesel-cli
        pkgs.bash
        pkgs.coreutils
        pkgs.findutils
        pkgs.openssl
        pkgs.postgresql.lib
        pkgs.cacert
        pkgs.libiconv
      ];

      # Extra commands to run when building the image
      extraCommands = ''
        mkdir -p app/migrations
        cp -r ${./migrations}/* app/migrations/
        cp ${entrypoint-script} app/entrypoint.sh
        chmod +x app/entrypoint.sh
      '';

      # Docker image configuration
      config = {
        Cmd = [ "/app/entrypoint.sh" ];
        Env = [
          "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
          "PATH=/bin:${hxckr-core}/bin:${pkgs.diesel-cli}/bin:${pkgs.findutils}/bin"
          "LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
            pkgs.openssl
            pkgs.postgresql.lib
            pkgs.libiconv
          ]}"
        ];
        WorkingDir = "/app";
        ExposedPorts = {
          "4925/tcp" = {};
        };
      };
    };
in
{
  # Build images for AMD64 and ARM64
  amd64 = buildImage "amd64" hxckr-core-amd64;
  arm64 = buildImage "arm64" hxckr-core-arm64;
}
