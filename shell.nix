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
    marksman
  ];
  shellHook = ''
    ./install-hooks.sh
  '';
}
