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
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        naersk' = pkgs.callPackage naersk {};

        quicksearch-rs = naersk'.buildPackage {
          pname = "quicksearch-rs";
          version = "0.1.0";
          src = ./.;
          buildInputs = [ pkgs.sqlite ];
        };
      in
      {
        formatter = pkgs.alejandra;

        packages = {
          quicksearch-rs = quicksearch-rs;
          default = quicksearch-rs;
        };

        devShell = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
          ];

          RUST_SRC_PATH =
            "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

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
          name = "quicksearch-rs";
          tag = "latest";

          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [
              quicksearch-rs
              pkgs.bash
              pkgs.curl
              pkgs.cacert
              pkgs.coreutils
            ];
            pathsToLink = [ "/bin" ];
          };
          config.Env = [
            "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
          ];
          config.Cmd = [ "quicksearch-rs" ];
        };
      }
    )
    // {
      # Optional but very useful: expose an overlay
      overlays.default = final: prev: {
        quicksearch-rs =
          self.packages.${final.system}.quicksearch-rs;
      };
    };
}
