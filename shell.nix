{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = [
    pkgconfig
    clang
    lld # To use lld linker
  ];
  buildInputs = [
    udev
    alsaLib
    vulkan-loader
    xlibsWrapper
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi # To use x11 feature
    libxkbcommon
    wayland # To use wayland feature
    wasm-bindgen-cli
    lld
    openssl
    wasm-pack
    nodePackages.typescript
  ];
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      pkgs.lib.makeLibraryPath [
        stdenv.cc.cc.lib
        udev
        alsaLib
        vulkan-loader
        libxkbcommon
        wayland # To use wayland feature
      ]
    }"'';
}
