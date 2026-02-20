{
  pkgs ? import <nixpkgs> { },
}:
let
  manifest = pkgs.lib.fromTOML (builtins.readFile ./Cargo.toml);
  version = manifest.package.version;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "templato";
  inherit version;
  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
