// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the graii results

use std::{convert::Infallible, env::VarError, net::AddrParseError, num::ParseIntError};

/// the graii errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("block {0} not found")]
    BlockNotFound(u64),
    #[error("no endpoints provided")]
    EmptyEndpoints,
    #[error("invalid path")]
    InvalidPath,
    #[error("invalid block range")]
    InvalidRange,
    #[error("invalid timestamp")]
    InvalidTimestamp,
    #[error("no block exists")]
    NoBlockExists,
    #[error("could not find data directory on this machine")]
    NoDataDirectory,
    #[error("no block has been marked as latest block rn")]
    NoLatestBlockRecord,
    #[error("parse block failed")]
    ParseBlockFailed,
    #[error("parse block ptr failed")]
    ParseBlockPtrFailed,
    #[error(transparent)]
    AddrParseError(#[from] AddrParseError),
    #[error(transparent)]
    Base64Decode(#[from] base64_url::base64::DecodeError),
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
    #[error(transparent)]
    Infallible(#[from] Infallible),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Timestamp(#[from] prost_types::TimestampOutOfSystemRangeError),
    #[error(transparent)]
    Uint(#[from] uint::FromDecStrErr),
    #[error(transparent)]
    Var(#[from] VarError),
}

/// result type
pub type Result<T> = std::result::Result<T, Error>;
