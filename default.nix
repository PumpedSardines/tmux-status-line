{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  frameworks = pkgs.darwin.apple_sdk.frameworks;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    buildInputs = [
      frameworks.Security
      frameworks.CoreFoundation
      frameworks.CoreServices
      frameworks.ApplicationServices
      frameworks.SystemConfiguration
      frameworks.CoreVideo
      frameworks.AppKit
    ];
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
  }
