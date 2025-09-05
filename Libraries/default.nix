{
  pkgs ? import <nixpkgs> { },
}:
with pkgs;

let
    buildDeps = with pkgs; [
        makeWrapper
        rustc
        cargo
        rust-analyzer
        rustfmt
        clippy
    ];

    runtimeDeps = with pkgs; [
        libffi
        libunwind
        vulkan-loader
    ] ++ lib.optionals stdenv.isDarwin [
        apple-sdk_15
    ] ++ lib.optionals stdenv.isLinux [
        wayland
        wayland-protocols
        wayland-scanner
        xorg.libX11
        xorg.libXrandr
        xorg.libXcursor
        xorg.libXi
        libxkbcommon
        xorg.libXext
        libdecor
        alsa-lib
        libGL
        libdrm
        libusb1
    ];

    libraryPath = pkgs.lib.makeLibraryPath runtimeDeps;
in
mkShell.override { } {
    nativeBuildInputs = with pkgs; [
    ] ++ buildDeps;

    buildInputs = with pkgs; [
    ] ++ runtimeDeps;

    shellHook = ''
        if [ "$(uname)" = "Darwin" ]; then
            export DYLD_LIBRARY_PATH=${libraryPath}:$DYLD_LIBRARY_PATH
        else
            export LD_LIBRARY_PATH=${libraryPath}:$LD_LIBRARY_PATH
        fi
    '';
}
