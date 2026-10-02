# omni-fixture

The test program of the contract scenarios and the QA harness (`crates/omni-fixture`, W01). It behaves the same
on every OS, prints only ASCII markers, and never sleeps to synchronize: scenarios wait for its markers. Every marker
line is written with a single write call, so lines from different fixture processes sharing a pipe never interleave.

```text
omni-fixture <step> [<step> ...]
```

Each argument is one step, run in order. A step is `verb` or `verb=value`. After the last step the fixture exits
0. An unknown verb or a bad value prints `FIXTURE-ERROR <reason>` to stderr and exits 99. Text values accept the
escapes `\n \r \t \\ \xHH` (`\xHH` writes a raw byte, so invalid UTF-8 is expressible).

| Verb | Does |
|---|---|
| `out=<text>` / `err=<text>` | write to stdout / stderr, flushed |
| `bytes=<stdout\|stderr>:<n>` | write `n` bytes of `abcdefghijklmnopqrstuvwxyz` repeated, no newline |
| `lines=<stdout\|stderr>:<n>` | write `n` lines `line <i>\n` (i from 1) |
| `ready` | print `READY\n` |
| `sleep=<ms>` · `hang` | sleep `ms` · sleep forever |
| `exit=<code>` | exit now with `code` (u32; on Windows the full 32-bit value, e.g. `3221225786`) |
| `signal=<NAME>` | Unix: raise `NAME` (e.g. `SIGTERM`) with the default disposition |
| `argv` | print the **remaining** arguments as a JSON array of strings and exit 0 (they are not steps) |
| `env` | print the environment as one JSON object, keys sorted |
| `cwd` | print the working directory |
| `cat` · `count-stdin` | copy stdin to stdout until EOF · read stdin to EOF, print `STDIN <bytes>` |
| `read-line` · `prompt=<text>` | read one line, print `GOT <line>` · print `text` (no newline), then as `read-line` |
| `close-stdin` | close its stdin |
| `ignore-term` | ignore graceful stop: Unix SIGTERM and SIGHUP; Windows CTRL_BREAK, CTRL_C and CTRL_CLOSE |
| `on-term=<text>` | on a graceful stop request: print `text`, exit 0 |
| `tree=<n>[:resist]` | start a chain of `n` descendants (child, grandchild, ...); each prints `PID <level> <pid>` (level 1..n) once running, then hangs; `:resist` makes them `ignore-term` |
| `hold=<ms>` | start one descendant that keeps the fixture's stdout and stderr open for `ms`, printing nothing |
| `escape` | Unix: start a descendant that calls `setsid`, prints `ESCAPED <pid>`, hangs. Windows: try `CREATE_BREAKAWAY_FROM_JOB`; print `ESCAPED <pid>` if it was created, else `ESCAPE-REFUSED` |
| `pidlog=<path>` | append its pid and a newline to `path` (the QA orphan oracle) |
| `tty` | print `{"stdin":bool,"stdout":bool,"stderr":bool,"cols":n,"rows":n}` (size 0 x 0 without a terminal) |
| `sizes` | print `SIZE <cols> <rows>` now and on every terminal resize, until stdin ends |

Descendants are the fixture itself (same binary), started without a shell, inheriting stdio unless stated.

W01 owns this file after W00 and may add a verb when an Appendix A item needs it; every verb added is listed here.
