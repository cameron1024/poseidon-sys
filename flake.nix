{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      targets = ["x86_64-unknown-linux-gnu" "aarch64-apple-darwin"];

      rustToolchain = pkgs.rust-bin.nightly."2023-01-10".default.override {
        extensions = ["rust-src" "rust-analyzer"];
        inherit targets;
      };

      cargoBuilds = map (t: "cargo build --release --target ${t};") targets;
      tarInputs = map (t: "mkdir libs/${t} && cp ${t}/release/libposeidon.a libs/${t};") targets;

      buildScript = pkgs.writeShellScriptBin "build" ''

        cd poseidon-impl

        ${pkgs.lib.concatStrings cargoBuilds}

        cd target

        rm -rf libs
        rm -rf libs.tar.gz

        mkdir libs
        ${pkgs.lib.concatStrings tarInputs}

        tar -czvf libs.tar.gz libs/**
      '';
    in {
      devShells.default = pkgs.mkShell {
        packages = [
          buildScript
          rustToolchain
        ];

        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
      };
    });
}
