
let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  executable = pkgs.rustPlatform.buildRustPackage rec {
    name = "ls";
    version = "0.0.1";
    src = ./.;
    cargoSha256 = "125xargnm3vfcj9743kglc15dgjg29cba212mq8zyl8npb0z8qji";
  };
  ls = pkgs.runCommand "ls" {} ''
     mkdir -p $out/bin
     cp ${executable}/bin/ls $out/bin/ls
     '';
in ls
