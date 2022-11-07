{
  description = "A basic flake with a shell";

  inputs = {
    nixpkgs.url = "nixpkgs";
    utils.url = "flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShell = with pkgs; mkShell rec {
          # for compilers and etc
          nativeBuildInputs = [
            pkg-config
          ];
          # for runtime dependencies
          buildInputs = [
            openssl
          ];
        };
      });
}
