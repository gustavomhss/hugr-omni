//! Host side of a terminal child: reading the PTY output, the `Go` handshake once the reader runs
//! (ADR-0005 R9), `resize`. W12. Its internal seam is frozen by the lead before W12 is dispatched.
