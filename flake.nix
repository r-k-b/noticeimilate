{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.my-project = naersk-lib.buildPackage {
        pname = "my-project";
        root = ./.;
      };
      defaultPackage = packages.my-project;

      # `nix run`
      apps.my-project = utils.lib.mkApp {
        drv = packages.my-project;
      };
      defaultApp = apps.my-project;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          docker-compose
          elmPackages.elm
          elmPackages.elm-format
          elmPackages.elm-live
          elmPackages.elm-test
          nodejs
          rustc

          # Seems we need rustup for IntelliJ tooling?
          # Had to run `rustup install stable` before IntelliJ could run
          # analyses on Rust code.
          # Does this risk diverging from the version that rustc / cargo
          # supply above?
          rustup
        ];
      };
    });
}
