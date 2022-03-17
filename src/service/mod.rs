// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Env, Result, Storage};
use async_trait::async_trait;

mod checking;
mod polling;

pub use self::{checking::Checking, polling::Polling};
use futures::lock::Mutex;
use std::sync::Arc;

#[async_trait]
pub trait Service: Sized {
    const NAME: &'static str;

    /// new service instance
    async fn new(env: &Env, storage: Arc<Mutex<Storage>>) -> Result<Self>;

    /// run service
    async fn run(&mut self) -> Result<()>;

    /// start service
    async fn start(&mut self) -> Result<()> {
        log::info!("start {} service...", Self::NAME);

        loop {
            if let Err(e) = self.run().await {
                log::warn!("{} service is down {}, restarting...", Self::NAME, e);
            }
        }
    }
}