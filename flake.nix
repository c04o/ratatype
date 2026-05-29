{
  description = "ratatype devshell";

  inputs = {
    # Nix Packages collection & NixOS
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Pure and reproducible nix overlay of binary distributed rust toolchains
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    supportedSystems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
  in {
    devShells = forAllSystems (
      system: let
        # apply overlay to pkgs
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [
            "x86_64-pc-windows-gnu"
            "x86_64-apple-darwin"
          ];
        };
      in {
        default = pkgs.mkShell {
          # tools needed to run on host
          nativeBuildInputs = with pkgs; [
            # cargo, rustc, rustfmt, clippy, rust-analyzer,...
            rustToolchain
            pkg-config

            # cross-compilation
            zig
            cargo-zigbuild
          ];

          # libraries app might link against
          buildInputs = with pkgs; [];
          shellHook = ''
            echo "ratatype devshell loaded!"
            echo "rust: $(rustc --version)"
          '';
        };
      }
    );
  };
}
