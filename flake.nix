{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      {
        packages.backend = pkgs.rustPlatform.buildRustPackage rec {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [
            openssl
          ];
        };

        packages.setup_vscode = pkgs.writeShellApplication {
          name = "setup_vscode";
          runtimeInputs = with pkgs; [
            direnv
          ];

          text = builtins.readFile ./setup_dev_env/setup_vscode.sh;
        };

        packages.default = pkgs.writeShellApplication {
          name = "app";
          runtimeInputs = with pkgs; [
            cargo
            nodejs_26
          ];
          text = builtins.readFile ./build_and_run.sh;
        };

        devShells.default = pkgs.mkShellNoCC {
          inputsFrom = [ self.packages.${system}.backend ];
          name = manifest.name + "_dev_shell";
          buildInputs = with pkgs; [
            rust-analyzer
            rustfmt
            clippy
            zsh
            zsh-completions
            nixfmt
          ];
          RUST_LOG = "info";
        };
      }
    );
}
