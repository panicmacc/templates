{
  description = "rust-yew project template";

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
        name = "rust-yew";
        #pkgs = nixpkgs.legacyPackages.${system}; 

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustChannel = "nightly";

        # `buildInputs` is for runtime dependencies. They need to match the target architecture.
        buildInputs = with pkgs; [
          # alsa-lib
          # alsaLib
          # atk
          # bevy 
          # egl-wayland
          # gdk-pixbuf
          # glew-egl
          # glib
          # gsettings-desktop-schemas
          # gtk3
          # libclang
          # libGL
          # libxkbcommon
          # mesa
          # openssl
          # pkgconfig
          # soapyaudio
          # soapyhackrf
          # soapyremote
          # soapyrtlsdr
          # soapysdr-with-plugins
          # udev
          # vulkan-headers
          # vulkan-loader
          # vulkan-tools
          # vulkan-validation-layers        
          # wayland
          # xlibsWrapper
          # xorg.libxcb
          # xorg.libX11
          # xorg.libXcursor
          # xorg.libXi
          # xorg.libXrandr
        ];

        # `nativeBuildInputs` is for build dependencies. They need to matchthe build host architecture.
        #  These get automatically added to PATH at build time.
        nativeBuildInputs = with pkgs; [
          binaryen
          cargo
          cargo-binutils
          cargo-edit
          jq
          # nodejs
          # openssl.dev
          # pkgconfig
          (rust-bin.${rustChannel}.latest.default.override {
            extensions = [
              "llvm-tools-preview"
              "rust-src" 
            ];
            targets = [
              # "thumbv7em-none-eabihf" # nrf52 / micro:bit v2
              "wasm32-unknown-unknown" 
              # "wasm32-wasi"
            ];
          })
          # python3
          rustup
          # speechd
          trunk
          unzip
          # vulkan-loader
          wasm-bindgen-cli
          wasm-pack
          zip
        ];

      in rec {

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
            # rust-bin.${rustChannel}.latest.rust-analysis
            # vulkan-tools
          ]);
          #
          RUST_SRC_PATH = "${pkgs.rust-bin.${rustChannel}.latest.rust-src}/lib/rustlib/src/rust/library";
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH"
            export PATH="$HOME/.cargo/bin:$PATH"
          '';

        };
      }
    );



}
