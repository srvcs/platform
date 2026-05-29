# srvcs Service Standard

This is the shared contract for a srvcs.cloud microservice. It is deliberately
small, repeatable, and more ceremony than the service usually deserves.

## Required Surface

Every service must expose:

- `GET /`: JSON identity payload.
- `GET /healthz`: liveness probe, always cheap.
- `GET /readyz`: readiness probe, safe for traffic gating.
- `GET /metrics`: Prometheus metrics.
- `GET /openapi.json`: current OpenAPI document.

Every service must support:

- JSON request logs on stdout.
- Request ID generation and propagation through `x-request-id`.
- Panic-to-500 handling.
- SIGTERM graceful shutdown.
- Apache-2.0 licensing.

## Runtime Configuration

The template supports these environment variables:

- `SRVCS_BIND_ADDR`: bind address, default `0.0.0.0:8080`.
- `SRVCS_ENV`: environment label for logs, default `development`.
- `RUST_LOG`: tracing filter, default `info,tower_http=info`.

## Local Gates

Native host checks:

```sh
nix flake check -L
nix develop -c sh -euc 'cargo fmt --check; cargo clippy --all-targets -- -D warnings; cargo test'
nix build .#default -L
```

The OCI image is a Linux-only flake package, so non-Linux hosts should build it
inside Linux Nix. On Apple Silicon, use `linux/arm64` for the practical local
check; CI builds the release image on native `x86_64-linux`.

```sh
docker run --rm --platform linux/arm64 -v "$PWD":/workspace -w /workspace nixos/nix:latest \
  sh -lc 'nix --extra-experimental-features "nix-command flakes" build .#container -L --out-link oci-image && cp -L oci-image image.tar.gz'
```

If Docker rejects Nix sandbox setup with a seccomp error, retry with
`--privileged` and add `--option sandbox false --option filter-syscalls false` to
the Nix command.

CI repeats these gates on Linux and smoke-tests `/healthz` from the built image.

## CI Contract

Services should call the shared workflow:

```yaml
jobs:
  build:
    uses: srvcs/platform/.github/workflows/build-service.yml@v1
    with:
      image-name: ${{ github.repository }}
      publish: true
```

`image-name` is required. Publishing only happens on `main` and `v*` tags, even
when callers set `publish: true`.

The workflow builds the binary, builds the container, runs the health smoke test,
pushes GHCR images, uploads an SBOM, and attests provenance for published images.

## Starting A Service

```sh
nix flake init -t github:srvcs/platform#service
```

After initialization, replace the `REPLACE-ME` OCI labels in `flake.nix` with the
repository name and source URL. Rename the Rust package and binary when the
service is ready to have an identity beyond the template.
