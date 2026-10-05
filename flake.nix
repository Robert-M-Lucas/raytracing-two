{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs = {
      type = "github";
      owner = "NixOS";
      repo = "nixpkgs";
      ref = "nixos-26.05";
    };
    nixpkgs-cmake = {
      type = "github";
      owner = "NixOS";
      repo = "nixpkgs";
      ref = "nixos-23.05";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSupportedSystem = f:
      inputs.nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
              inputs.self.overlays.default
            ];
          };
          pkgsCmake = import inputs.nixpkgs-cmake {
            inherit system;
          };
        });
  in {
    overlays.default = final: prev: {
      # rustToolchain = ...
    };

    devShells = forEachSupportedSystem ({
      pkgs,
      pkgsCmake,
    }: {
      default = pkgs.mkShell {
        packages = with pkgs;
          [
            pkg-config
            SDL2
          ]
          ++ [pkgsCmake.cmake];

        # We still need this even with 'bundled', as statically compiled SDL2
        # still dynamically loads host display drivers at runtime on Linux.
        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (with pkgs; [
          libGL
          libglvnd
          vulkan-loader
          wayland
          libxkbcommon
          libx11
          libxcursor
          libxrandr
          libxi
        ])}:/run/opengl-driver/lib:/run/opengl-driver-32/lib";

        env = {
          # Required by rust-analyzer
          # RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          SDL_VIDEODRIVER="wayland";
        };
      };
    });
  };
}
