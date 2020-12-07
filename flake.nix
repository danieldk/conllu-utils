{
  description = "CoNLL-U utilities";

  inputs = {
    naersk = {
      url = "github:nmattia/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/release-20.09";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: {
      defaultPackage = naersk.lib.${system}.buildPackage ./.;
        
      defaultApp = {
        type = "app";
        program = "${self.defaultPackage.${system}}/bin/conllu";
      };
    });
}
