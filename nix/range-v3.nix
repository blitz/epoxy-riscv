{ stdenv, fetchFromGitHub, cmake, ninja }:
stdenv.mkDerivation rec {
  pname = "range-v3";
  version = "0.10.0";

  src = fetchFromGitHub {
    owner = "ericniebler";
    repo = pname;
    rev = version;

    sha256 = "1h9h5j7pdi0afpip9ncq76h1xjhvb8bnm585q17afz2l4fydy8qj";
  };

  nativeBuildInputs = [ cmake ninja ];

  cmakeFlags = [
    "-DRANGE_V3_DOCS=OFF"
    "-DRANGE_V3_TESTS=OFF"
    "-DRANGE_V3_EXAMPLES=OFF"
    "-DRANGE_V3_PERF=OFF"
  ];
}
