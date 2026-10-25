# vyges-suite

The Vyges engines as two binaries:

| Binary | Engines |
|---|---|
| `vyges-loom` | the analysis and sign-off engines, and the loom front end |
| `vyges-physical` | the physical construction engines |

Each binary runs the engine it is invoked as. `vyges install loom` / `vyges install physical`
download one binary and link every engine's name to it, so `vyges-sta-si …`, `vyges sta-si …` and
`vyges loom sta-si …` all keep working. A binary can also be given the engine as its first
argument: `vyges-physical pdn …`.

`vyges-physical --suite-engines` lists the engines a binary carries.

## Building

The engines are built from sibling checkouts (`../vyges-tools-<engine>`), the layout the release
workflow creates at the release tag:

```bash
cargo build --release --features loom --bin vyges-loom
cargo build --release --features physical --bin vyges-physical
```

`vyges-physical` links the prebuilt solver archives its engines use; see
`.github/workflows/release.yml` for how they are fetched.

## License

Apache-2.0. See `LICENSE` and `NOTICE`.
