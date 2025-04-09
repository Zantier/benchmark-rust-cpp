{
  inputs = {
    # nixpkgs 882842d2a908700540d206baa79efb922ac1c33d
    #   gcc 13.3.0, make 4.4.1
    nixpkgs.url = "github:nixos/nixpkgs/882842d2a908700540d206baa79efb922ac1c33d";
  };

  outputs = { nixpkgs, ... }:
    let
      systems = [ "aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux" ];
      forEachSystem = function:
        nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});
    in {
      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.gcc
            pkgs.gnumake
          ];
        };
      });
    };
}
