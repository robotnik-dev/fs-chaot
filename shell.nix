# {
#   pkgs ? import <nixpkgs> { },
# }:
# let
#   overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
# in
# pkgs.callPackage (
#   {
#     atkmm,
#     cairo,
#     gcc,
#     gdk-pixbuf,
#     glib,
#     gtk3,
#     mkShell,
#     openssl,
#     pango,
#     pkg-config,
#     rustup,
#     rustPlatform,
#     stdenv,
#     webkitgtk_4_1, # for javascriptcoregtk-rs-sys
#     xdotool, # for libxdo
#   }:
#   mkShell {
#     strictDeps = true;
#     nativeBuildInputs = [
#       gcc
#       openssl
#       pkg-config
#       rustup
#       rustPlatform.bindgenHook
#     ];
#     # libraries here
#     buildInputs = [
#       atkmm
#       cairo
#       gdk-pixbuf
#       glib
#       gtk3
#       pango
#       webkitgtk_4_1
#       xdotool
#     ];
#     GDK_BACKEND = "x11"; # NVIDIA might disagree otherwise.
#     PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
#     RUSTC_VERSION = overrides.toolchain.channel;
#     WEBKIT_DISABLE_DMABUF_RENDERER = 1; # Again NVIDIA things.
#     # https://github.com/rust-lang/rust-bindgen#environment-variables
#     shellHook = ''
#       export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
#       export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
#     '';
#   }
# ) { }
