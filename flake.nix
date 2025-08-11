{
  description = "Rust + Dioxus environment (base simplified config)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];

        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            allowUnfree = true;
          };
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [
            "wasm32-unknown-unknown"
            "aarch64-linux-android"
            "armv7-linux-androideabi"
            "i686-linux-android"
            "x86_64-linux-android"
          ];
        };

        dioxusCli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "dioxus-cli";
          version = "0.6.3";
          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-wuIJq+UN1q5qYW4TXivq93C9kZiPHwBW5Ty2Vpik2oY=";
          };
          cargoHash = "sha256-L9r/nJj0Rz41mg952dOgKxbDS5u4zGEjSA3EhUHfGIk=";
          nativeBuildInputs = [ pkgs.pkg-config pkgs.cacert ];
          buildInputs = [ pkgs.openssl ];
          OPENSSL_NO_VENDOR = 1;
          doCheck = false;
        };

        rustBuildInputs = [
          pkgs.openssl
          pkgs.rustup
          pkgs.libiconv
          pkgs.pkg-config
          pkgs.mesa
          pkgs.libgbm
          pkgs.libglvnd
          pkgs.xorg.libXi
          pkgs.xorg.libXrandr
          pkgs.xorg.libX11
        ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
          pkgs.glib
          pkgs.gtk3
          pkgs.libsoup_3
          pkgs.webkitgtk_4_1
          pkgs.xdotool
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
          pkgs.darwin.apple_sdk.frameworks.IOKit
          pkgs.darwin.apple_sdk.frameworks.Carbon
          pkgs.darwin.apple_sdk.frameworks.WebKit
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.Cocoa
        ]);


      in
      {
        devShells.default = pkgs.mkShell {
          name = "dioxus-web-shell";

          buildInputs = [
            rustToolchain
            dioxusCli
          ] ++ rustBuildInputs;

          env = { };

        };
      });
}
