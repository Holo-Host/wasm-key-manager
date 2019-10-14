{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  inherit (rust.packages.nightly) rustPlatform;
in

{
  hclient2 = buildRustPackage rustPlatform rec {
    name = "hclient2";
    src = gitignoreSource ./.;
    cargoDir = ".";

    nativeBuildInputs = with buildPackages; [
      nodejs-12_x
      pkgconfig
      (wasm-pack.override { inherit rustPlatform; })
    ];

    buildInputs = [ openssl ];

    buildPhase = ''
      cp -r ${npmToNix { src = "${src}/${cargoDir}"; }} node_modules
      chmod -R +w node_modules
      chmod +x node_modules/.bin/webpack
      patchShebangs node_modules

      npm run build
    '';

    installPhase = ''
      mv target/webpack $out
    '';

    doCheck = false;
  };
}
