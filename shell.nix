{ pkgs ? import (builtins.fetchTarball https://github.com/nixos/nixpkgs/archive/20.09.tar.gz) { }
, rust-overlay ? (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)
}:

with import "${rust-overlay}/rust-overlay.nix" pkgs pkgs;
let
  rustNightly = rustChannelOf { date = "2020-10-17"; channel = "nightly"; };
in
pkgs.mkShell {
  buildInputs = with pkgs;
    [
      rustNightly.rust
      pkgconfig
      openssl
    ];

  RUST_BACKTRACE = 0;
  RUST_SRC_PATH = "${rustNightly.rust-src}/lib/rustlib/src/rust/library";
}
