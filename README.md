# srvcs/platform

The srvcs service framework: the definition of a srvcs microservice, a Nix flake
**template** to stamp new services from, and one **centralized CI workflow** every
service reuses.

- What a service is: see [`STANDARD.md`](./STANDARD.md).
- Start a new service: `nix flake init -t github:srvcs/platform#service`.
