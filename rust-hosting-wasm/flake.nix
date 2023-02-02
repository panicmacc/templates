{
  description = "rust-plugins-wasm project starter";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };
    utils = { url = "github:numtide/flake-utils"; };
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
    flake-compat = {
      url = "github:edolstra/flake-compat"; 
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }:
  
    utils.lib.eachDefaultSystem (system:
    
      let
        name = "rust-plugins-wasm";
        #pkgs = nixpkgs.legacyPackages.${system}; 

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustChannel = "nightly";

        # `buildInputs` is for runtime dependencies. They need to match the target architecture.
        buildInputs = with pkgs; [
          # alsa-lib
          clang
          # egl-wayland
          # glew-egl
          # glib
          # libclang
          # libGL
          # libxkbcommon
          # mesa
          # openssl
          # pkgconfig
          # vulkan-loader
          # wasmedge
          # wayland
          # xorg.libxcb
          # xorg.libX11
          # xorg.libXi
          # bevy 
          # udev
          # alsaLib
          #xlibsWrapper
          # xorg.libXcursor
          # xorg.libXrandr
          # xorg.libXi
          # vulkan-tools
          # vulkan-headers
          # vulkan-loader
          # vulkan-validation-layers        
          #
          #for blackjack
          # atk
          # gsettings-desktop-schemas
          # gtk3
          # gdk-pixbuf
          #
          #for soapysdr
          # libclang
          # soapyaudio
          # soapyhackrf
          # soapyremote
          # soapyrtlsdr
          # soapysdr-with-plugins
        ];

        # `nativeBuildInputs` is for build dependencies. They need to matchthe build host architecture.
        #  These get automatically added to PATH at build time.
        nativeBuildInputs = with pkgs; [
          binaryen
          cargo
          cargo-binutils
          cargo-edit
          cargo-embed
          jq
          # libclang
          nodejs
          openssl.dev
          pkgconfig
          (rust-bin.${rustChannel}.latest.default.override {
            extensions = [
              "llvm-tools-preview"
              "rust-src" 
            ];
            targets = [
              "thumbv7em-none-eabihf" # nrf52 / micro:bit v2
              "wasm32-unknown-unknown" 
              "wasm32-wasi"
            ];
          })
          python3
          rustup
          speechd
          spin
          trunk
          unzip
          vulkan-loader
          wasm-bindgen-cli
          wasm-pack
          wasmedge
          zip
        ];

      in rec {
        # `flattenTree` returns a flat list of the package's derivations, ignoring other attribs.
        packages = utils.lib.flattenTree {
          hello = pkgs.hello;
          gitAndTools = pkgs.gitAndTools;
          
          # TODO: set up real build with `buildRustPackage` and make this dynamic e.g. `packages.${name}`
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
          ]);
          # Here you can add any tools you need present in your development environment, 
          #  but that may not be needed at build or runtime. 
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            cargo-watch
            nixpkgs-fmt
            rust-analyzer
            rust-bin.${rustChannel}.latest.rust-analysis
            vulkan-tools
          ]);
          #
          RUST_SRC_PATH = "${pkgs.rust-bin.${rustChannel}.latest.rust-src}/lib/rustlib/src/rust/library";
          WASMEDGE_INCLUDE_DIR = "${pkgs.wasmedge}/include";
          WASMEDGE_LIB_DIR = "${pkgs.wasmedge}/lib";
          LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
          # for the bevy
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH"
            export PATH="$HOME/.cargo/bin:${pkgs.wasmedge}/bin:$PATH"
          '';

        };
        # 
        #shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
        #  pkgs.alsaLib
        #  pkgs.udev
        #  pkgs.vulkan-loader
        #]}"'';
      }
    );



}
