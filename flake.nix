{
  description = "Build and development environment for Typsite";

  outputs =
    {
      self,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forEachSystem =
        f:
        builtins.listToAttrs (
          map (system: {
            name = system;
            value = f system (import ./default.nix { inherit system; });
          }) systems
        );
    in
    {
      packages = forEachSystem (
        system: nix: rec {
          typsite = nix.package;
          default = typsite;
        }
      );

      devShells = forEachSystem (
        system: nix: {
          default = nix.devShell;
        }
      );
    };
}
