with import <nixpkgs> {};

mkShell {
  buildInputs = [
    cargo
    openssl
  ];
  shellHook = ''
    export OPENSSL_DIR="${openssl.dev}"
    export OPENSSL_LIB_DIR="${openssl.out}/lib"
  '';
}
