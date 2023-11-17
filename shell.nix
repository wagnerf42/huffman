{ pkgs ? import <nixpkgs> { } }:
let
  fenix = import
    (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz")
    { };
in pkgs.mkShell {
  buildInputs = with pkgs; [

    (with fenix;
     combine (with stable; [
        cargo
        clippy-preview
        latest.rust-src
        rust-analyzer
        rust-std
        rustc
        rustfmt-preview
      ]))
  ];
}
