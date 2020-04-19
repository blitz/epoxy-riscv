{ stdenv, fetchFromGitHub, cmake, ninja }:
stdenv.mkDerivation rec {
  pname = "pprintpp";
  version = "1.0.0";

  src = fetchFromGitHub {
    owner = "tfc";
    repo = pname;

    rev = version;
    sha256 = "0zznrvcpn2dqgnrxb3lh91dwxih7km1lcx76747lnwqylrvyg20s";
  };
  
  nativeBuildInputs = [ cmake ninja ];
}
