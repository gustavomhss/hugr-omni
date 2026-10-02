//! The public surface (`docs/api-contract.md`). Frozen in W00: names, signatures and docs change only by a
//! lead decision recorded in PLAN. Types are defined next to the module that implements them and
//! re-exported here, so this module stays a thin, readable table of contents.

mod command;

pub use crate::io::{Chunk, Data, Line, Lines, Output, Stream};
pub use crate::process::{Child, DroppedBytes, Exit, PipeChild, ProcessInfo, PtyChild, Reason, RunOutput};
pub use crate::spawn::{PtySize, Stdin};
pub use command::Command;
pub use tokio_util::sync::CancellationToken;
