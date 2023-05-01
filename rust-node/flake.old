{
  description = "rust-basic project starter";

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
        name = "rust-node";
        #pkgs = nixpkgs.legacyPackages.${system}; 

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };

        rustChannel = "nightly";

        # `buildInputs` is for runtime dependencies. They need to match the target architecture.
        buildInputs = with pkgs; [
          alsa-lib
          glib
          libGL
          libxkbcommon
          mesa
          openssl.dev
          vulkan-loader
          wayland
          xorg.libxcb
          xorg.libX11
          xorg.libXi
          # Adding these to try to get bevy to build
          pkgconfig
          udev
          alsaLib
          x11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          vulkan-tools
          vulkan-headers
          vulkan-loader
          vulkan-validation-layers        
          #for blackjack
          atk
          gsettings-desktop-schemas
          gtk3
          gdk-pixbuf
        ];

        # `nativeBuildInputs` is for build dependencies. They need to matchthe build host architecture.
        #  These get automatically added to PATH at build time.
        nativeBuildInputs = with pkgs; [
          binaryen
          cargo
          cargo-make
          jq
          nodejs
          pkgconfig
          (rust-bin.${rustChannel}.latest.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          })
          speechd
          trunk
          vulkan-loader
          wasm-bindgen-cli
          wasm-pack
          python3
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
            rust-bin.${rustChannel}.latest.rls
            vulkan-tools
          ]);
          #
          RUST_SRC_PATH = 
            "${pkgs.rust-bin.${rustChannel}.latest.rust-src}/lib/rustlib/src/rust/library";
          # for the bevy
          shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
            pkgs.alsaLib
            pkgs.udev
            pkgs.vulkan-loader
          ]}"'';

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
