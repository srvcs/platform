# srvcs service

Template Rust + Nix microservice for srvcs.cloud.

## Local checks

```sh
nix flake check -L
nix develop -c sh -euc 'cargo fmt --check; cargo clippy --all-targets -- -D warnings; cargo test'
nix build .#default -L
```

The Linux container is exposed as `.#container`. On Apple Silicon, use
`linux/arm64` for the practical local check; CI builds the release image on
native `x86_64-linux`.

```sh
docker run --rm --platform linux/arm64 -v "$PWD":/workspace -w /workspace nixos/nix:latest \
  sh -lc 'nix --extra-experimental-features "nix-command flakes" build .#container -L --out-link oci-image && cp -L oci-image image.tar.gz'
```

If Docker rejects Nix sandbox setup with a seccomp error, retry with
`--privileged` and add `--option sandbox false --option filter-syscalls false` to
the Nix command.

See [`srvcs/platform`](https://github.com/srvcs/platform) for the shared service
standard and CI workflow.
