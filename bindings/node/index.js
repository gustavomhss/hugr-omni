// W00 stub so the TS runner (W02) can load the package and fail every scenario for a stated reason.
// W13 replaces this file with the real loader of the native module. No process logic ever lives here.
"use strict";

class OmniError extends Error {
  constructor(code, message, result) {
    super(message);
    this.name = "OmniError";
    this.code = code;
    if (result !== undefined) this.result = result;
  }
}

const notYet = () => new OmniError("IO", "IO: not implemented yet (W13: native binding)");

function run() {
  return Promise.reject(notYet());
}

function spawn() {
  throw notYet();
}

module.exports = { run, spawn, OmniError };
