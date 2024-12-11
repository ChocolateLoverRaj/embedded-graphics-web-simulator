{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit (pkgs) lib;
        rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        });
        linker = with pkgs; {
          "x86_64-linux" = mold;
          "x86_64-darwin" = zld;
          "aarch64-darwin" = zld;
        };
      in {
        devShells.default = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            rust
            rust-analyzer
            clang

            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr # To use the x11 feature
            libxkbcommon
            wayland # To use the wayland feature

            httplz
            nil
            wasm-bindgen-cli
          ];

          nativeBuildInputs = with pkgs;
            [pkg-config] ++ lib.catAttrs system [linker];

          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

          shellHook = ''
          '';
        };
      }
    );
}
