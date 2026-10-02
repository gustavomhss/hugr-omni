//! `run()`: spawn + collect, with the completion rules of contract §6. W09 (collection is `io`, W10).

use super::Options;
use crate::error::{Error, ErrorCode};
use crate::spawn::Request;
use crate::types::RunOutput;

/// Runs to completion (contract §6).
pub(crate) async fn run(req: &Request, opts: &Options) -> Result<RunOutput, Error> {
    let _ = (req, opts);
    Err(Error::new(ErrorCode::Io, "not implemented yet (W09: process/run)"))
}
