{
  description = "A dead simple UDP to DMX transmitter";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rust;

        src = craneLib.cleanCargoSource (craneLib.path ./.);

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;

          buildInputs = with pkgs; [
            pkg-config
            libusb1
          ];
        };

        netdmx = (craneLib.buildPackage {
          inherit src;

          strictDeps = true;

          inherit cargoArtifacts;

          buildInput = with pkgs; [
            libusb1
          ];
        });
      in
      {
        checks = {
          inherit netdmx;
        };

        packages = rec {
          inherit netdmx;
          default = netdmx;
          deps = cargoArtifacts;
        };

        apps = rec {
          netdmx = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "netdmx" ''
              ${netdmx}/bin/netdmx
            '';
          };
          default = netdmx;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          buildInputs = with pkgs; [
            pkg-config
            libusb1
          ];

          RUST_BACKTRACE = 1;
          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };
      });
}
