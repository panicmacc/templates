{
  description = "rust-axum project starter";

  inputs = {
    start.url = "github:panicmacc/start";
    nixpkgs.follows = "start/nixpkgs-stable";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.follows = "start/flake-utils";
  };

  outputs = { self, start, nixpkgs, nixpkgs-unstable, utils }@inputs:
  
    utils.lib.eachDefaultSystem (system:
    
      let

        #########
        # HW Acceleration Support
        #

        # Enabling CUDA support increases build time
        #   `$ cachix use cuda-maintainers` may help
        #
        cudaSupport = true;

        #########
        # Global settings
        #
        
        pkgsUnstable = import nixpkgs-unstable {
          inherit system;
          config = { allowUnfree = true; };
        };
          
        # Cherry-pick some things from unstable
        stable_unstable_things = self: super: with pkgsUnstable; {
          inherit blender cmctl glooctl postman super-slicer kicad;
        };
        
        overlays = [
          start.overlays.rust-overlay.overlays.default
          stable_unstable_things
        ];

        pkgs = import nixpkgs {
          inherit overlays system;
          config = { allowUnfree = true; };
        };

        ##########
        # Rust Overlay Configuration  
        #
          
        rustChannel = "nightly";
        
        # rustVersion = (pkgs.rust-bin.${rustChannel}.latest.default.override {
        #   extensions = [ "rust-src" ];
        #   targets = [ "wasm32-unknown-unknown" ];
        # });
            
        # rustPlatform = pkgs.makeRustPlatform {
        #   cargo = rustVersion;
        #   rustc = rustVersion;
        # };

        nixosGitVersionStamp = (
          { pkgs, ... }: {
            system.configurationRevision = pkgs.lib.mkIf (self ? rev) self.rev;
          }
        );

        ##########
        # Python Configuration  
        #

        python-packages = p: with p; [
          flake8
          pandas
          pip
          setuptools
          (if cudaSupport then torchWithCuda else torch)
          wheel
        ];

        # `buildInputs` is for runtime dependencies. They need to match the target architecture.
        buildInputs = with pkgs; [
          #cudatoolkit
	        fontconfig
          #libGL
          libtorch-bin
          #mesa
          openssl
	        onnxruntime
          pkgconfig
          #gcc-unwrapped.lib # CUDA
          stdenv.cc.cc.lib 
	        zlib
          cudatoolkit
	        cudaPackages.cudnn
	        cudaPackages.libcublas
	        cudaPackages.libcufft
	        cudaPackages.libcurand
          libGL
          mesa
          gcc-unwrapped.lib
        ];

        cudaBuildInputs = with pkgs; [
        ];

        # `nativeBuildInputs` is for build dependencies. They need to match the build host architecture.
        #  These get automatically added to PATH at build time.
        nativeBuildInputs = with pkgs; [
          binaryen
          cargo
          cargo-binutils
          cargo-edit
          cargo-embed
	        cmake
          gcc
          jq
          micromamba
          nodejs
          openssl.dev
          pkgconfig
          poetry
          (python310.withPackages python-packages)
          (rust-bin.${rustChannel}.latest.default.override {
            extensions = [
              "llvm-tools-preview"
              "rust-src" 
            ];
            targets = [
              #"thumbv7em-none-eabihf" # nrf52 / micro:bit v2
              "wasm32-unknown-unknown" 
              "wasm32-wasi"
            ];
          })
          rustup
          unzip
          wasm-bindgen-cli
          wasm-pack
          zip 
        ];

      in rec {
        # `flattenTree` returns a flat list of the package's derivations, ignoring other attribs.
        packages = utils.lib.flattenTree {
          hello = pkgs.hello;
          gitAndTools = pkgs.gitAndTools;
          
          # https://github.com/NixOS/nixpkgs/blob/4fc53b59aecbc25c0e173163d60155f8fca14bfd/doc/languages-frameworks/rust.section.md#compiling-rust-applications-with-cargo 

          #rust-simple = rustPlatform.buildRustPackage rec {
          #  pname = "${name}";
          #  version = "0.0.1";
          #
          #  src = fetchFromGitHub {
          #    owner = "panicmacc";
          #    repo = pname;
          #    rev = version;
          #    sha256 = "1iga3320mgi7m853la55xip514a3chqsdi1a1rwv25lr9b1p7vd3";
          #  };
          #
          #  cargoSha256 = "17ldqr3asrdcsh4l29m3b5r37r5d0b3npq1lrgjmxb6vlx6a36qh";
          #
          #  meta = with stdenv.lib; {
          #    description = "A snazzy Rust app.";
          #    homepage = "https://github.com/panicmacc/rust-simple";
          #    license = licenses.unlicense;
          #    maintainers = [ maintainers.tailhook ];
          #  };
          #};
        };

        # `nix build` 
        defaultPackage = packages.hello;

        # `nix run`
        apps.hello = utils.lib.mkApp { drv = packages.hello; };
        defaultApp = apps.hello;

        # `nix develop`
        devShell = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.packages.${system};
          buildInputs = buildInputs ++ (with pkgs; [
          ]) ++ (if cudaSupport then cudaBuildInputs else []);
          # Here you can add any tools you need present in your development environment, 
          #  but that may not be needed at build or runtime. 
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            cargo-watch
            nixpkgs-fmt
            rust-analyzer
          ]);
          #
          RUST_SRC_PATH = "${pkgs.rust-bin.${rustChannel}.latest.rust-src}/lib/rustlib/src/rust/library";
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH"
            export PATH="$HOME/.cargo/bin:$PATH"
          '';
            #export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.stdenv.cc.cc.lib}/lib" # Still needed even with makeLibraryPath?
        };
      }
    );

}
