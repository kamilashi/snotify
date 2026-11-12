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
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
                    pname = manifest.name;
                    version = manifest.version;
                    cargoLock.lockFile = ./Cargo.lock;
                    src = pkgs.lib.cleanSource ./.;
                    nativeBuildInputs = with pkgs; [ pkg-config ]; 
                    buildInputs = with pkgs; [
                      openssl 
                    ];
                  };

        devShells.default = pkgs.mkShellNoCC {
          inputsFrom = [ self.packages.${system}.default ];
          name = manifest.name + "_dev_shell";
          buildInputs = with pkgs; [
            rust-analyzer 
            rustfmt       
            clippy
            zsh       
            zsh-completions
          ];
          RUST_LOG="info";
        };
      }
    );
}
