{
  description = "";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };
  outputs = { self, nixpkgs, flake-utils, pre-commit-hooks }:
    flake-utils.lib.eachSystem [ flake-utils.lib.system.x86_64-linux ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        l = builtins // pkgs.lib;
        inverseInclude = list: (l.filter (fileName: l.any (e: e != fileName) list) (l.attrNames (l.readDir ./.)));
      in
      {
        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
          packages = with pkgs; [
            nodejs-16_x
            mdbook
            statix
            nodePackages.cspell
            nodePackages.markdownlint-cli
          ];
          shellHook = ''
            ${self.checks.${system}.pre-commit-check.shellHook}
          '';
        };
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              cspell = {
                enable = true;
                entry = "${pkgs.nodePackages.cspell}/bin/cspell --words-only";
                types = [ "markdown" ];
              };
              markdownlint.enable = true;
            };
          };
        };
        packages = {
          docs = pkgs.runCommand
            "static-docs"
            { nativeBuildInputs = [ pkgs.mdbook ]; }
            ''
              mdbook build -d $out ${./.}/docs
              echo typednix.dev > $out/CNAME
            '';

          nix-types = pkgs.rustPlatform.buildRustPackage {
            pname = "nix-types";
            version = "0.1.0";
            src = ./parser;

            cargoLock = {
              lockFile = ./parser/Cargo.lock;
              outputHashes = {
                # "arenatree-0.1.1" = "";
                # "rnix-0.4.1" = "";
              };
            };
            nativeBuildInputs = [ pkgs.pkg-config ];
          };
          # currently use nixpkgs/lib as test source
          parsed = pkgs.stdenv.mkDerivation {
            name = "test-data";
            src = nixpkgs;
            nativeBuildInputs = [ self.packages.${system}.nix-types ];
            buildPhase = ''
              echo "running nix metadata collect in nixpkgs/lib"
              ${self.packages.${system}.nix-types}/bin/nix-types --dir ./lib
            '';
            installPhase = ''
              cat data.json > $out
            '';
          };
          default = self.packages.${system}.parsed;
        };
      });
}
