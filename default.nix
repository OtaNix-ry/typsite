{
  sources ? import ./npins,
  system ? builtins.currentSystem,
  pkgs ? import sources.nixpkgs {
    inherit system;
    config = { };
    overlays = [ (import sources.rust-overlay) ];
  },
}:
let
  projectName = "typsite";
  projectVersion = "0.1.6";

  commonRustFlags = [
    "--cfg"
    "tokio_unstable"
  ];

  rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [
      "rust-src"
      "clippy"
      "rustfmt"
    ];
  };

  customRustPlatform = pkgs.makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };

  projectSrc = pkgs.lib.cleanSourceWith {
    src = ./.;
    filter =
      path: type:
      let
        baseName = baseNameOf (toString path);
      in
      !(pkgs.lib.elem baseName [
        ".git"
        "target"
        "result"
        ".direnv"
        ".envrc"
        "tests"
      ])
      && !(pkgs.lib.hasSuffix ".nix" baseName)
      && !(pkgs.lib.hasSuffix ".lock" baseName && baseName != "Cargo.lock");
  };

  typsite = customRustPlatform.buildRustPackage {
    pname = projectName;
    version = projectVersion;

    src = projectSrc;

    cargoLock.lockFile = ./Cargo.lock;
    nativeBuildInputs = [
      pkgs.pkg-config
    ];
    RUSTFLAGS = pkgs.lib.concatStringsSep " " commonRustFlags;

    meta = with pkgs.lib; {
      description = "Typsite";
      homepage = "https://typsite.skillw.com";
      license = licenses.mit;
    };
  };

in
{
  package = typsite;

  devShell = pkgs.mkShell {
    name = "${projectName}-dev";

    packages = [
      rustToolchain
      pkgs.rust-analyzer

      pkgs.pkg-config
      pkgs.openssl
      pkgs.git
      pkgs.bashInteractive

      typsite
    ];

    shellHook = ''
      echo "Entering ${projectName} (v${projectVersion}) development environment..."
      echo "Rust toolchain: $(rustc --version)"
      echo "Typst version: $(typst --version || echo 'Typst not found in PATH immediately, but available')"

      export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"

      echo ""
      echo "To build the project with Nix: nix-build"
      echo "The binary will be in ./result/bin/${projectName}"
      echo ""
      echo "For iterative development, use standard cargo commands:"
      echo "  cargo build"
      echo "  cargo run"
      echo "  cargo test"
      echo "  cargo clippy"
      echo ""
    '';
  };

  formatter = pkgs.nixpkgs-fmt;
}
