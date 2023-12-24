{ pkgs ? import <nixpkgs> {} }:

# While not needed this is left here in case I want to show my solution for day 24 part 2
# Namely this includes z3 and libclang (idk if i need z3 itself tbh) so it can compile

pkgs.mkShell {
    name = "advent-shell";
    buildInputs = with pkgs; [
        rustc
        rust-bindgen
        cargo
        clippy
        rustfmt
        clang
        cmake
        libclang
        gcc
        z3
        pkg-config
    ];
    shellHook = ''
        export LIBCLANG_PATH=${pkgs.libclang.lib}/lib
    '';
}
