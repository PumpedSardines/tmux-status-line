{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    nixpkgsUnstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flakeUtils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    nixpkgsUnstable,
    flakeUtils,
  }:
    flakeUtils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        pkgsUnstable = nixpkgsUnstable.legacyPackages.${system};
        packages = with self.packages.${system}; [
          cargo
          rustc
          rustfmt
          openssl
          pkg-config
        ];
      in {
        packages = flakeUtils.lib.flattenTree {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
          rustfmt = pkgs.rustfmt;
          openssl = pkgs.openssl;
          pkg-config = pkgs.pkg-config;
        };

        devShell = pkgs.mkShell {
          buildInputs = packages;
        };
      }
    );
}
