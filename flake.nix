{
  description = "Basic Flake to install Templato";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        package = pkgs.callPackage ./. { inherit pkgs; };
      in
      {
        packages = {
          default = package;
        };
      }
    );
}
