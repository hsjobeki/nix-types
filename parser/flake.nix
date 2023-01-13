{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
  };
  outputs = {self, flake-utils, nixpkgs, ...}@inp:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {inherit system;};   
      in rec {
        devShell = pkgs.mkShell {
           inputsFrom = [packages.nix-types];
           
        };
        packages = {
          nix-types = pkgs.rustPlatform.buildRustPackage {
            pname = "nix-types";
            version = "0.1.0";
            src = ./.;
            
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "arenatree-0.1.1" = "sha256-b3VVbYnWsjSjFMxvkfpJt13u+VC6baOIWD4qm1Gco4Q=";
                "rnix-0.4.1" = "sha256-C1L/qXk6AimH7COrBlqpUA3giftaOYm/qNxs7rQgETA=";
              };
            };
            nativeBuildInputs = [ pkgs.pkg-config ];
          };
          # currently use nixpkgs/lib as test source
          parsed = pkgs.stdenv.mkDerivation {
            name = "test-data";
            src = nixpkgs;
            nativeBuildInputs = [packages.nix-types];
            buildPhase = ''
              echo "running nix metadata collect in nixpkgs/lib"
              ${packages.nix-types}/bin/nix-types --dir ./lib
            '';
            installPhase = ''
              cat data.json > $out  
            '';
          };
        default = packages.parsed;
        };
      }
    );
}