{
  description = "python-simple project starter";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };
    utils = { url = "github:numtide/flake-utils"; };
    flake-compat = {
      url = "github:edolstra/flake-compat"; 
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        name = "python-basic";

        pkgs = import nixpkgs {
          inherit system;
        };

        python-packages = p: with p; [
          flake8
          pandas
          pip
          setuptools
          wheel
        ];

        # `buildInputs` is for runtime dependencies. They need to match the target architecture.
        buildInputs = with pkgs; [
          openssl
          stdenv.cc.cc.lib
          gcc-unwrapped.lib
        ];

        # `nativeBuildInputs` is for build dependencies. They need to matchthe build host architecture.
        #  These get automatically added to PATH at build time.
        nativeBuildInputs = with pkgs; [
          pkg-config
          poetry
          (python310.withPackages python-packages)
          gcc
          micromamba
        ];

      in rec {
        # `flattenTree` returns a flat list of the package's derivations, ignoring other attribs.
        packages = utils.lib.flattenTree {
          hello = pkgs.hello;
          gitAndTools = pkgs.gitAndTools;
        };

        # `nix build` 
        #defaultPackage = packages.hello;

        # `nix run`
        #apps.hello = utils.lib.mkApp { drv = packages.hello; };
        #defaultApp = apps.hello;

        # `nix develop`
        devShell = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.packages.${system};
          buildInputs = buildInputs ++ (with pkgs; [
          ]);
          # Here you can add any tools you need present in your development environment, 
          #  but that may not be needed at build or runtime. 
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
          ]);
          shellHook = ''
            export PATH="$PATH"
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.stdenv.cc.cc.lib}/lib"
          '';
        };
      }
    );



}
