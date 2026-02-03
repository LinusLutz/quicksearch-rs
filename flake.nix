{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
        naersk' = pkgs.callPackage naersk {};
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          buildInputs = [pkgs.sqlite];
        };
      in {
        formatter = nixpkgs.legacyPackages.${system}.alejandra;
        packages.default = defaultPackage;
        devShell = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = ["rust-src"];
            })
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          nativeBuildInputs = with pkgs; [
            rust-bin.beta.latest.default
            cargo
            clippy
            rust-analyzer
            rustc
            openssl.dev
            pkg-config
          ];
        };
        images.default = pkgs.dockerTools.buildImage {
          name = "dotkinder";
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [
              defaultPackage
              pkgs.bash
              pkgs.curl
              pkgs.cacert
              pkgs.coreutils
            ];
            pathsToLink = ["/bin"];
          };
          config.Cmd = ["quicksearch-rs"];
        };
      }
    );
}
