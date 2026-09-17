# Registry publication — prepared, NOT submitted

Research current as of 2026-09-17, from zed.dev publishing docs and
`zed-industries/extensions@main`.

## Current process (as documented today)

Publishing is a PR to <https://github.com/zed-industries/extensions>. The process
changed from a plain manifest entry to **git submodules**:

1. Fork `zed-industries/extensions` **to a personal account** (Zed staff may push fixes
   to your PR; org forks block that).
2. `git submodule init && git submodule update` in the clone.
3. Add this repo as a submodule at `extensions/<id>`; the URL must be **HTTPS** (not
   `git@github.com:`), and the pinned commit must be reachable on a branch (no detached
   commits).
4. Add an entry to the top-level `extensions.toml`; `version` must match this repo's
   `extension.toml` at the pinned commit.
5. Run `pnpm sort-extensions`.
6. Open the PR. Rules: exactly one extension per PR; at most 3 open PRs per submitter;
   respond to maintainer feedback within 3 weeks or the PR is closed.

## Prepared submission

Once this repo is public at `https://github.com/mpapis/zed-extension-ripwire-lsp`:

```sh
git submodule add https://github.com/mpapis/zed-extension-ripwire-lsp extensions/ripwire-lsp
git add extensions/ripwire-lsp
```

`extensions.toml` entry:

```toml
[ripwire-lsp]
submodule = "extensions/ripwire-lsp"
version = "0.1.0"
```

then `pnpm sort-extensions`.

## Compliance checklist (verified 2026-09-17)

- [x] ID `ripwire-lsp`: kebab-case; no `zed`/`extension`; `-lsp` suffix as required for
      language-server-only extensions.
- [x] No collision: no `ripwire` or `ripwire-lsp` entry in `extensions.toml` on
      `main` (grep of the full 6,021-line manifest, 2026-09-17).
- [x] License: Apache-2.0, on the accepted list; `LICENSE` at repo root.
- [x] No bundled language server — binary resolved from the user's environment
      (`RIPWIRE_PATH` → PATH), per the "do not bundle" rule.
- [x] `schema_version = 1` (current docs' example uses 1).
- [x] User-facing text is English.
- [ ] Manual test inside Zed at the pinned commit — **required before submitting**; do a
      dev-extension install and confirm all five capabilities.
- [ ] Icon assets: not required by manifest; Zed falls back to a generic icon.

## Do not submit without explicit go-ahead from Michal.
