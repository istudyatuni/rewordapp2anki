{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = with pkgs; [
      cargo-xwin
      sqlite
    ];
}
