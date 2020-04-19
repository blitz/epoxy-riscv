{ sources ? import ./nix/sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:

let thisPackage = import ./default.nix { inherit sources nixpkgs pkgs; };
in thisPackage.riscvPkgs.mkShell {
  inputsFrom = [ thisPackage.kernel ];

  nativeBuildInputs = pkgs.lib.attrsets.mapAttrsToList (_: v: v) thisPackage.shellDependencies;
}
