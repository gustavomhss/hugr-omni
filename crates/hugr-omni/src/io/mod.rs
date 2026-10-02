//! Output pumps with bounded buffering, UTF-8 decoding, lines, stdin, and `run()` collection (W10).
//!
//! SEAM (frozen in W00): the public types re-exported by `api`. Everything else belongs to W10.

mod out;

pub use out::{Chunk, Data, Line, Lines, Output, Stream};
