use async_trait::async_trait;
use dapr::proto::{runtime::v1 as dapr_v1};
use tonic::{transport::Channel as TonicChannel};

use crate::dapr::*;
use crate::error::Error;

pub struct Client<T>(T);

impl<T: DaprConnector> Client<T> {
    /// Connect to a Dapr enabled app.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address of gRPC server to connect to.
    pub async fn connect(addr: String) -> Result<Self, Error> {
        Ok(Client(T::connect(addr).await?))
    }
}

#[async_trait]
pub trait DaprConnector: Sized {
    async fn connect(addr: String) -> Result<Self, Error>;
}

#[async_trait]
impl DaprConnector for dapr_v1::dapr_client::DaprClient<TonicChannel> {
    async fn connect(addr: String) -> Result<Self, Error> {
        Ok(dapr_v1::dapr_client::DaprClient::connect(addr).await?)
    }
}

/// A tonic based gRPC client
pub type TonicClient = dapr_v1::dapr_client::DaprClient<TonicChannel>;