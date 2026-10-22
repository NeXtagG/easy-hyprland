# For non-flake users: nix-build, or `pkgs.callPackage ./nix/package.nix { }`.
{
  pkgs ? import <nixpkgs> { },
}:
pkgs.callPackage ./nix/package.nix { }
