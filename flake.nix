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
      in
      {
        packages.${system} = {
            snotify_rust = pkgs.rustPlatform.buildRustPackage rec {
                      pname = "snotify_rust";
                      version = "0.1";
                      cargoLock.lockFile = ./Cargo.lock;
                      src = pkgs.lib.cleanSource ./.;
                    }
            default = self.packages.${system}.snotify_rust;
        };

        devShells.${system}.default = pkgs.mkShellNoCC {
          name = "snotify_dev_shell";
        };
      }
    );
}
