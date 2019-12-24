let thisPackage = import ./default.nix { };
in thisPackage.riscvPkgs.mkShell {
  inputsFrom = [ thisPackage.kernel ];
  nativeBuildInputs = [ thisPackage.bootScript thisPackage.pkgs.niv ];
}
