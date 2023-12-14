{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        name = "entity";
        shell-hook = shell-name: ''
          PS1="\n\[\033[01;32m\]${name}(${shell-name}) >\[\033[00m\] "
        '';
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            rust-analyzer
            rust-bin.nightly.latest.default
            gdb
            # Viewing images
            image-roll
          ];

          shellHook = shell-hook "default";
        };
      }
    );
}
