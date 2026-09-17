# Ripwire LSP — Zed extension

Registers [ripwire](https://github.com/redhat-et/ripwire)'s `--lsp` stdio server in
[Zed](https://zed.dev), providing read-only whole-tree navigation:

- go to definition
- find references
- document symbols (outline)
- workspace symbols
- hover

Supported languages: C, C++, Python, Shell Script, Markdown, JavaScript, TypeScript,
TSX, JSON, YAML, Rust, Java, Elixir, Ruby.

This extension does **not** bundle ripwire. It only resolves a ripwire binary already on
your machine and starts it with `--lsp`.

## Prerequisites

One of, in order of precedence:

1. `lsp.ripwire-lsp.binary.path` set in Zed settings — Zed consults this before the
   extension.
2. `RIPWIRE_PATH` set in your shell environment.
3. `ripwire` on your `PATH`.

The binary must be **ripwire ≥ 0.6.1** (the release that added `--lsp`).

## Install

### From the registry

Not yet published. See [PUBLISHING.md](PUBLISHING.md) for the prepared submission. Once
live, install it from Zed's extension marketplace.

### Dev extension (current path)

In Zed: `zed: install dev extension` and select this directory. Or symlink it manually:

```sh
mkdir -p ~/.local/share/zed/extensions/installed
ln -s "$PWD" ~/.local/share/zed/extensions/installed/ripwire-lsp
```

Restart Zed. Check `zed: open log` (`Zed.log`) if the server does not start.

## Rebuild the wasm

There is no Rust toolchain requirement on the host; builds run in an openSUSE Leap 16.0
container. One-time image bake (installs rustup with pinned rust 1.98.0, the
wasm32-wasip2 target, clippy, rustfmt):

```sh
docker run --name ripwire-bake -d leap-build sleep 3600
docker exec -e RUSTUP_HOME=/opt/rustup -e CARGO_HOME=/opt/cargo ripwire-bake \
  sh -c 'curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal \
    --default-toolchain 1.98.0 --target wasm32-wasip2 --component clippy,rustfmt'
docker commit ripwire-bake leap-build:wasip2
docker rm -f ripwire-bake
```

Then build, check, and update the committed artifact:

```sh
docker run --rm -u 1000:1000 -e HOME=/tmp \
  -e CARGO_HOME=/tmp/cargo -e RUSTUP_HOME=/opt/rustup \
  -e PATH=/opt/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
  -v "$HOME/.cache/leap-cargo:/tmp/cargo:z" -v "$PWD:$PWD:z" -w "$PWD" \
  leap-build:wasip2 sh -c '
    cargo fmt --check &&
    cargo clippy --release --target wasm32-wasip2 -- -D warnings &&
    cargo build --release --target wasm32-wasip2 &&
    cp target/wasm32-wasip2/release/zed_extension_ripwire_lsp.wasm extension.wasm'
```

(On SELinux hosts the `:z` mount suffix is required.)

### Reproducibility

The committed `extension.wasm` is byte-reproducible **with this exact recipe**: rust
1.98.0 (official rustup builds, `88d9e12ae 2026-08-18`), target `wasm32-wasip2`, built
from the committed `Cargo.lock`. Clean rebuilds in the pinned toolchain have produced
identical bytes, including from scratch with a different target directory:

```text
sha256  6fe6f352ec7470dc9bafb48c9323fdfffb853937339593e4f3f5958df4cc4b04
```

Byte equality across *different* toolchain versions or non-rustup distributions is not
claimed. If you rebuild with a different compiler you may get a different artifact.
CI rebuilds with the pinned toolchain and fails if the artifact drifts.

## Honest floors

Results inherited from ripwire: symbol and reference counts are **floors** — lower bounds
of what is found in indexed files. Unsaved buffers (editors' in-memory state) are absent
from the index; ripwire sees only files on disk.

## Provenance

This extension's Rust code was generated entirely by an LLM session. The author is not
fluent in Rust and cannot personally vouch for its idiomaticity. Treat it accordingly
until a Rust-literate review lands. `cargo fmt`/`clippy -D warnings` pass; the surface is
one trait implementation.

Issues: open them on this repository. Upstream ripwire issues belong to
[redhat-et/ripwire](https://github.com/redhat-et/ripwire). Security reports: use GitHub's
private vulnerability disclosure on this repository
(Security tab → "Report a vulnerability").

## License

Apache-2.0. Parent project ripwire is Apache-2.0; the linked `zed_extension_api` is
Apache-2.0.
