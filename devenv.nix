{pkgs, ...}: {
  packages = with pkgs; [
    # Rust toolchain
    rustup
    clang
    cmake
    pkg-config

    # Code formatting tools
    treefmt

    # Dependency auditing
    cargo-deny

    # Additional toolchain components
    cargo-leptos
    podman-compose
  ];

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
