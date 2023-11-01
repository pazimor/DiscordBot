{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec {
    buildInputs = with pkgs; [ cargo rustc rustup clang llvmPackages.bintools ];
}

##libcap2-bin

## TODO: ajouter a la build "sudo setcap cap_net_raw=eip"
