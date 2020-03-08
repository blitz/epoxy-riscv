let thisPackage = import ./default.nix { };
in thisPackage.riscvPkgs.mkShell {
  inputsFrom = [ thisPackage.kernel ];

  nativeBuildInputs = with thisPackage; [
    bootScript
    dhall
    pkgs.clang-tools
    pkgs.niv
    pkgs.nixfmt
  ];
}
