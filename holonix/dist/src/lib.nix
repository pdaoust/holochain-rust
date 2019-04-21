let
 pkgs = import ../../nixpkgs/nixpkgs.nix;
 rust = import ../../rust/config.nix;
 dist = import ../config.nix;
 git = import ../../git/config.nix;
in
{
 binary-derivation = args:
  pkgs.stdenv.mkDerivation {
   name = "holochain-${args.name}";

   src = pkgs.fetchurl {
    url = "https://github.com/${git.github.repo}/releases/download/v${dist.version}/${args.name}-v${dist.version}-${dist.artifact-target}.tar.gz";
    sha256 = args.sha256;
   };

   unpackPhase = ":";

   installPhase = ''
     mkdir -p $out/bin
     cp $src $out/bin/${args.binary}
     # chmod +x $out/share/${args.binary}
     # ln -s $out/share/${args.binary} $out/bin/${args.binary}
     ls -la $out/bin/${args.binary}
     # cp $src $out/bin/${args.binary}
   '';
  };

}
