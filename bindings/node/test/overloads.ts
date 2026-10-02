// C-TS-01, the types an idiom relies on: the PipeChild / PtyChild overloads, `OmniError.code` as a closed set, the
// `reason` union and `await using` / `for await` on a Child. Checked by `tsc -p test/tsconfig.json` against the frozen
// index.d.ts; nothing here runs. A `@ts-expect-error` that stops being an error is itself an error, so every line that
// says "wrong" can fail.
import { OmniError, run, spawn, type Child, type Exit, type PipeChild, type PtyChild, type RunResult, type SpawnOptions } from "hugr-omni";

declare const options: SpawnOptions;
declare const err: OmniError;

const pipe: PipeChild = spawn("x");
const piped: PipeChild = spawn("x", ["y"], { stdin: "pipe" });
const pty: PtyChild = spawn("x", [], { pty: true });
const sized: PtyChild = spawn("x", undefined, { pty: { cols: 100, rows: 30 } });
const either: PipeChild | PtyChild = spawn("x", [], options);
const shared: Child = pty;
const result: Promise<RunResult> = run("x", ["y"], { pty: true, timeoutMs: 1 });

void pipe.closeStdin();
pty.resize(80, 24);

// @ts-expect-error a pipe child has no resize
pipe.resize(80, 24);
// @ts-expect-error a terminal has no closeStdin
pty.closeStdin();
// @ts-expect-error `pty` never gives a PipeChild
const notPipe: PipeChild = spawn("x", [], { pty: true });
// @ts-expect-error `stdin` is a spawn option, not a run option
void run("x", [], { stdin: "pipe" });
// @ts-expect-error `input` is a run option, not a spawn option
spawn("x", [], { input: "x" });

const code: "NOT_FOUND" | "NOT_EXECUTABLE" | "INVALID_CWD" | "INVALID_ARGUMENT" | "ABORTED" | "OUTPUT_LIMIT" | "CLOSED" | "IO" = err.code;
// @ts-expect-error the codes are a closed set
const closed: "NOPE" = err.code;
const asError: Error = err;
const partial: RunResult | undefined = err.result;

function reasonText(reason: Exit["reason"]): string {
  switch (reason) {
    case "exit":
    case "signal":
    case "killed":
    case "timeout":
    case "aborted":
      return reason;
    default: {
      const unreachable: never = reason;
      return unreachable;
    }
  }
}

await using scoped = spawn("x");
const disposable: AsyncDisposable = scoped;
for await (const chunk of scoped.output) {
  const stream: "stdout" | "stderr" | "pty" = chunk.stream;
  void stream;
  break;
}
for await (const line of scoped.lines()) {
  const text: string = line.text;
  void text;
  break;
}

export { code, asError, partial, reasonText, piped, sized, either, shared, result, notPipe, closed, disposable };
