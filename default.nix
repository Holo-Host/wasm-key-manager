{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  inherit (rust.packages.nightly) rustPlatform;
in

{
  wasm-key-manager = buildRustPackage rustPlatform rec {
    name = "wasm-key-manager";
    src = gitignoreSource ./.;
    cargoDir = ".";

    nativeBuildInputs = with buildPackages; [
      jq
      nodejs-12_x
      pkgconfig
      (wasm-pack.override { inherit rustPlatform; })
    ];

    buildInputs = [ openssl ];

    buildPhase = ''
      bash build.sh
    '';

    installPhase = ''
      mv pkg $out
    '';

    doCheck = false;
  };
}
