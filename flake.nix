{
  description = "PTZ Camera Controller";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          # Tools that run on your development machine
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];

          # Libraries that your Rust binary will link against
          buildInputs = with pkgs; [
            gtk4
            glib
            cairo
            pango
            graphene
            gdk-pixbuf
            libadwaita
          ];

          # Environment variables
          # Helps rust-analyzer find the Rust standard library sources
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      }
    );
}
