let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = [ pkgs.cargo pkgs.rustc pkgs.pkg-config pkgs.xdotool pkgs.rust-analyzer pkgs.alsa-lib pkgs.fltk pkgs.pango ];
}
