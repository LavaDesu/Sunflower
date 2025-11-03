{
  description = "nix dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/85dbfc7aaf52ecb755f87e577ddbe6dbbdbc1054";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };
    toolchain = pkgs.rust-bin.nightly."2025-08-19".default.override {
      targets = [
        "armv7-linux-androideabi"
        "i686-linux-android"
        "aarch64-linux-android"
        "x86_64-linux-android"
      ];
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        pkg-config
        openssl
        toolchain
      ];
    };
  };
}
