{ sources ? import ./nix/sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:

let thisPackage = import ./nix/release.nix { inherit sources nixpkgs pkgs; };
in thisPackage.riscvPkgs.mkShell {
  inputsFrom = [ thisPackage.newWorld.epoxy-hello ];

  nativeBuildInputs = pkgs.lib.attrsets.mapAttrsToList (_: v: v) thisPackage.shellDependencies;
  buildInputs = [ pkgs.niv ];
}
