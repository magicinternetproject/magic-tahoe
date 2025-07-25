{
  description = "magic_tahoe";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    crane.url = "github:ipetkov/crane";
    rustOverlay.url = "github:oxalica/rust-overlay";
    devshell.url = "github:numtide/devshell";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    rustOverlay,
    devshell,
    advisory-db,
  }: let
      forAllSystems = function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ] (system: function rec {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                (import rustOverlay)
                devshell.overlays.default
              ];
            };

            rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
              toolchain.default.override {
                extensions = ["rust-src"];
              });

            craneLib = crane.lib.${system}.overrideToolchain rustToolchain;

            src = craneLib.cleanCargoSource ./.;

            craneCommon = {
              inherit src;
              RUSTFLAGS = [
                # Lint groups
                ["-D" "clippy::correctness"]
                ["-D" "clippy::complexity"]
                ["-D" "clippy::perf"]
                ["-D" "clippy::style"]
                ["-D" "clippy::nursery"]
                ["-D" "clippy::pedantic"]
                # Allowed by default
                ["-D" "clippy::cognitive_complexity"]
                ["-D" "clippy::expect_used"]
                ["-D" "clippy::unwrap_used"]
                ["-D" "clippy::print_stderr"]
                ["-D" "clippy::print_stdout"]
                ["-D" "clippy::pub_use"]
                ["-D" "clippy::redundant_closure_for_method_calls"]
                ["-D" "clippy::single_char_lifetime_names"]
                ["-D" "clippy::str_to_string"]
                ["-D" "clippy::string_to_string"]
                ["-D" "clippy::unwrap_in_result"]
                ["-D" "clippy::wildcard_enum_match_arm"]
                # Allow certain rules
                ["-A" "clippy::missing_errors_doc"]
                ["-A" "clippy::module_name_repetitions"]
                # No warnings
                # ["-D" "warnings"]
              ];
            };

            cargoArtifacts = craneLib.buildDepsOnly craneCommon;

            magic_tahoe = craneLib.buildPackage (craneCommon
              // {
                inherit cargoArtifacts;
              });
            });
    in {
      checks = forAllSystems ({
        magic_tahoe,
        craneLib,
        craneCommon,
        cargoArtifacts,
        src, ...
      }: {
        inherit magic_tahoe;

        magic_tahoe-clippy = craneLib.cargoClippy (craneCommon
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets";
          });

        # Check formatting
        magic_tahoe-fmt = craneLib.cargoFmt {
          inherit src;
        };

        # Audit dependencies
        magic_tahoe-audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };
      });

      formatter = forAllSystems ({ pkgs, ... }: pkgs.alejandra);

      packages = forAllSystems ({ magic_tahoe, ... }: {
        inherit magic_tahoe;
        default = magic_tahoe;
      });

      apps = forAllSystems ({system, ...}: {
        magic_tahoe = {
          type = "app";
          program = "${self.packages.${system}.magic_tahoe}/bin/magic_tahoe";
        };
        default = self.apps.${system}.magic_tahoe;
      });

      devShells = forAllSystems ({
        craneCommon,
        rustToolchain,
        pkgs,
        ...
      }: {
        default = pkgs.devshell.mkShell {
        commands = let
          categories = {
            hygiene = "hygiene";
            development = "development";
          };
        in [
          {
            help = "Check rustc and clippy warnings";
            name = "check";
            command = ''
              set -x
              cargo check --all-targets
              cargo clippy --all-targets
            '';
            category = categories.hygiene;
          }
          {
            help = "Automatically fix rustc and clippy warnings";
            name = "fix";
            command = ''
              set -x
              cargo fix --all-targets --allow-dirty --allow-staged
              cargo clippy --all-targets --fix --allow-dirty --allow-staged
            '';
            category = categories.hygiene;
          }
          {
            help = "Run cargo in watch mode";
            name = "watch";
            command = "cargo watch";
            category = categories.development;
          }
        ];

        imports = ["${devshell}/extra/language/rust.nix"];
        language.rust = {
          packageSet = rustToolchain;
          tools = ["rustc"];
          enableDefaultToolchain = false;
        };

        devshell = {
          name = "magic_tahoe-devshell";
          packages = with pkgs; [
            # Rust build inputs
            clang
            coreutils

            # LSP's
            rust-analyzer

            # Tools
            cargo-watch
            rustToolchain
            alejandra
          ];
        };

        env = [
          {
            name = "RUSTFLAGS";
            eval = "\"${builtins.toString craneCommon.RUSTFLAGS}\"";
          }
        ];
        };
      });
    };
}
