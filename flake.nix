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
        devShell = with pkgs; mkShell {
          # for compilers and etc
          nativeBuildInputs = [
            sqlx-cli
            pkg-config
          ];
          # for runtime dependencies
          buildInputs = [
            openssl
          ];
          DATABASE_URL = "sqlite:todos.db";
          TELOXIDE_TOKEN = "5700014825:AAEesxB09g7EdM-LBPlb36mUidOx1rwr9ZE"; # revoked token
        };
      });
}
