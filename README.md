# hugr-omni

> **Status: planning (pre-alpha). Nothing is published yet.**

Run processes and terminals with identical, tested behavior on Windows, macOS and Linux —
from TypeScript (Node, Bun, Deno), Python and Rust. Built for AI agents and developer tools.

- `run` / `spawn` that resolve `npm` the same way everywhere, never use a shell, and kill the
  **whole process tree** on stop, timeout, cancel or scope exit.
- Pseudo-terminals (openpty / ConPTY) that never hang on exit.
- One native Rust core; thin, idiomatic packages per language; one shared conformance suite.

See [PLAN.md](PLAN.md) for the execution plan and [docs/acceptance.md](docs/acceptance.md)
for the contract and the KPIs that decide the first release.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT)
at your option.
