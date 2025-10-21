{
  inputs = {
    # Your preferred primary nix relesae
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    # Proivdes legacy compatibility for nix-shell
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    # Provides some nice helpers for multiple system compatibility
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      flake-compat,
    }:
    # Calls the provided function for each "default system", which
    # is the standard set.
    flake-utils.lib.eachDefaultSystem (
      system:
      # instantiate the package set for the supported system, with our
      # rust overlay
      let
        pkgs = import nixpkgs { inherit system; };
        # We define a new derivation by overriding attributes of an existing package
        # dioxus-cli-0_7_0_rc2 = pkgs.dioxus-cli.overrideAttrs (oldAttrs: rec {
        #   pname = "dioxus-cli";
        #   version = "0.7.0-rc.2";
        #   src = pkgs.fetchCrate {
        #     inherit pname version;
        #     hash = "sha256-Gri7gJe9b1q0qP+m0fe4eh+xj3wqi2get4Rqz6xL8yA=";
        #   };
        # });
      in
      # "unpack" the pkgs attrset into the parent namespace
      with pkgs;
      {
        devShell = mkShell {
          # Packages required for development.
          buildInputs = [
            # Add your system dependencies here
            openssl
            libiconv
            pkg-config
            glib
            gtk3
            libsoup_3
            webkitgtk_4_1
            xdotool
            rustPlatform.bindgenHook
            wasm-bindgen-cli_0_2_104
          ];
          shellHook = ''
            cargo binstall dioxus-cli@0.7.0-rc.3 -y
          '';
        };
      }
    );
}
