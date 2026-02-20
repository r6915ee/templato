{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  name = "templato";
  buildInputs = with pkgs; [
    rustc
    clippy
    rustfmt
    cargo
    rust-analyzer
    commitizen
  ];
  shellHook = ''
    ./install-hooks.sh
  '';
}
