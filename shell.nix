{ sources ? import ./nix/sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:

let local = import ./nix/release.nix { inherit sources nixpkgs pkgs; };
in local.rv64Pkgs.mkShell {
  inputsFrom = [ local.systems.qemu-hello-64.kern ];

  nativeBuildInputs = pkgs.lib.attrsets.mapAttrsToList (_: v: v) local.shellDependencies;
  buildInputs = [ pkgs.niv ];
}
