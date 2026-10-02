# Contract scenarios

One JSON file per scenario in `conformance/scenarios/`, written once (W01). The same file runs through every
runner (Rust: `crates/hugr-omni/tests/`, TS: `bindings/node/test/`; Python in v0.2). A runner only reads
scenarios; it never adds or skips one (`skipped` counts as a failure). The idiom items (C-TS-01, C-RS-01, C-PY-01,
C-HOST-01) are written per language beside the runner, not here.

```json
{
  "id": "C-KILL-01.after-wait",
  "os": ["linux", "macos", "windows"],
  "files": { "bin/tool.cmd": "@echo off\r\n\"${FIXTURE}\" argv %*\r\n" },
  "steps": [
    { "spawn": ["${FIXTURE}", "tree=2", "exit=0"], "as": "c" },
    { "read": "c", "until": { "match": "PID 2 ", "count": 1 }, "capture": { "pids": "PID \\d (\\d+)" } },
    { "processes": "c", "expect": { "entries": [
        { "pid": "${c.pid}", "parentPid": null, "name": { "regex": "^omni-fixture" } },
        { "pid": "${pids}", "parentPid": "${c.pid}", "name": { "regex": "^omni-fixture" } },
        { "pid": "${pids}", "parentPid": "${pids}", "name": { "regex": "^omni-fixture" } } ] } },
    { "wait": "c", "expect": { "reason": "exit", "exitCode": 0 } },
    { "os": "alive", "pids": "${pids}" },
    { "stop": "c", "graceMs": 1000, "expect": { "reason": "exit", "elapsedMs": { "lt": 1500 } } },
    { "os": "dead", "pids": "${pids}", "withinMs": 500 }
  ]
}
```

- **`id`** starts with the Appendix A item it proves (`C-KILL-01.`). `os` defaults to all three OSes; `langs` (`rust`,
  `ts`, `py`) defaults to all languages and is set only when an input cannot be expressed in a language's types (e.g.
  a NaN duration in Rust). A runner never skips a scenario meant for it.
- **`files`** are created under `${TMP}` before the first step (`\n` is written as is; use `\r\n` where needed).
- **Variables:**
  - `${FIXTURE}`: absolute path of `omni-fixture`;
  - `${TMP}`: a fresh directory per scenario;
  - `${EXE}`: `.exe` on Windows, empty elsewhere;
  - `${<handle>.pid}`: the pid of a spawned child;
  - captures: `${name}` is the list of every value captured under `name`; inside a string, the first one.
- **Number values** in `options` may be the tagged `{ "$number": "NaN" | "Infinity" | "-Infinity" }`.

## Steps (10)

Each step has exactly one action key. The value of the key is the handle name (`as` of an earlier step) or, for
`run`/`spawn`, `[command, ...args]`. Every step fails after 10 s unless it states `timeoutMs`; a timeout is a
failure, never a pass.

| Step | Does | Extra keys |
|---|---|---|
| `run` | `run(command, args, options)` | `options`, `as`, `await` (default `true`; `false` = start it and keep the promise under `as`), `expect` |
| `spawn` | `spawn(command, args, options)` | `options`, `as`, `expect` (an `error` here = thrown synchronously) |
| `write` | `write(data)` then, with `close: true`, `closeStdin()` | `data` (text, or `{ "hex": "…" }`), `close`, `expect` |
| `read` | claims (first time) and reads the output until `until`; the consumer stays claimed and paused between reads | `lines` (bool, default chunks), `until` (`{ "match": regex, "count": n }` or `"end"`), `detach` (leave the loop for good), `capture`, `expect` |
| `wait` | `wait()` of a child, or awaits a `run` started with `await: false` | `expect` |
| `stop` | `stop({ graceMs })` | `graceMs`, `expect` |
| `abort` | aborts the `AbortSignal` that every `run`/`spawn` of the runner gets | – |
| `resize` | `resize(cols, rows)` | `cols`, `rows`, `expect` |
| `processes` | `processes()` | `capture` (the pids), `expect` |
| `os` | asks the OS (never the library) that every pid in `pids` is `alive` or `dead` (gone or a zombie) | `pids`, `withinMs` (poll until true or fail) |

`options` uses the TS names of `docs/api-contract.md` (`cwd`, `env`, `inheritEnv`, `timeoutMs`, `graceMs`, `text`,
`mergeStderr`, `input`, `maxOutputBytes`, `stdin`, `pty`); each runner maps them to its language.

## Expectations

`expect` lists fields of the step's result; every listed field must match, and unlisted fields are ignored.
- **Thrown/rejected error:** `error` (code), `message` (substrings that must all appear), `result` (fields of
  `error.result`).
- **Exit / RunResult:** `exitCode`, `signal`, `reason`, `success`, `stdout`, `stderr`.
- **`read`:** `stdout`, `stderr`, `pty` (everything this step read from that stream, joined), `chunks` (count),
  `lostBefore` (sum seen), `droppedBytes` (`{ "stdout": …, "stderr": … }`), `continues` (count of pieces marked).
- **`processes`:** `entries`, an unordered list that must match the result one to one:
  `{ "pid": …, "parentPid": … | null, "name": matcher }`. `pid` and `parentPid` accept a number or a capture (which
  matches any of its values).
- **Lists of pids** (captures, `os` steps) are compared as sets.
- **Any step:** `elapsedMs` (from the step's start).

Matchers:
- **Strings:** an exact string, or `{ "contains": s }`, `{ "regex": r }`, `{ "length": n }`, or `{ "hex": "…" }`
  (the exact raw bytes; in text mode, those of the UTF-8 text).
- **Numbers:** an exact value, or `{ "lt" | "lte" | "gt" | "gte": n }` (several may combine).

A failure prints the scenario id, the step index, its action, and expected vs got.
