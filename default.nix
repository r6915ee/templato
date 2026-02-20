{
  pkgs ? import <nixpkgs> { },
}:
pkgs.rustPlatform.buildRustPackage {
  pname = "templato";
  version = "0.1.1";
  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
