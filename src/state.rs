impl<T: DaprState> Client<T> {
    /// Get the state for a specific key.
    ///
    /// # Arguments
    ///
    /// * `store_name` - The name of state store.
    /// * `key` - The key of the desired state.
    pub async fn get_state<S>(
        &mut self,
        store_name: S,
        key: S,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<GetStateResponse, Error>
    where
        S: Into<String>,
    {
        let mut mdata = HashMap::<String, String>::new();
        if let Some(m) = metadata {
            mdata = m;
        }

        self.0
            .get_state(GetStateRequest {
                store_name: store_name.into(),
                key: key.into(),
                metadata: mdata,
                ..Default::default()
            })
            .await
    }

    /// Save an array of state objects.
    ///
    /// # Arguments
    ///
    /// * `store_name` - The name of state store.
    /// * `states` - The array of the state key values.
    pub async fn save_state<I, K>(&mut self, store_name: K, states: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = (K, Vec<u8>)>,
        K: Into<String>,
    {
        self.0
            .save_state(SaveStateRequest {
                store_name: store_name.into(),
                states: states.into_iter().map(|pair| pair.into()).collect(),
            })
            .await
    }

    /// Delete an array of state objects.
    ///
    /// # Arguments
    ///
    /// * `store_name` - The name of state store.
    /// * `states` - The array of the state key values.
    pub async fn delete_bulk_state<I, K>(&mut self, store_name: K, states: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = (K, Vec<u8>)>,
        K: Into<String>,
    {
        self.0
            .delete_bulk_state(DeleteBulkStateRequest {
                store_name: store_name.into(),
                states: states.into_iter().map(|pair| pair.into()).collect(),
            })
            .await
    }

    /// Delete the state for a specific key.
    ///
    /// # Arguments
    ///
    /// * `store_name` - The name of state store.
    /// * `key` - The key of the desired state.
    pub async fn delete_state<S>(
        &mut self,
        store_name: S,
        key: S,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<(), Error>
    where
        S: Into<String>,
    {
        let mut mdata = HashMap::<String, String>::new();
        if let Some(m) = metadata {
            mdata = m;
        }

        self.0
            .delete_state(DeleteStateRequest {
                store_name: store_name.into(),
                key: key.into(),
                metadata: mdata,
                ..Default::default()
            })
            .await
    }
}

#[async_trait]
impl DaprState for dapr_v1::dapr_client::DaprClient<TonicChannel> {
    async fn get_state(&mut self, request: GetStateRequest) -> Result<GetStateResponse, Error> {
        Ok(self.get_state(Request::new(request)).await?.into_inner())
    }

    async fn save_state(&mut self, request: SaveStateRequest) -> Result<(), Error> {
        Ok(self.save_state(Request::new(request)).await?.into_inner())
    }

    async fn delete_state(&mut self, request: DeleteStateRequest) -> Result<(), Error> {
        Ok(self.delete_state(Request::new(request)).await?.into_inner())
    }

    async fn delete_bulk_state(&mut self, request: DeleteBulkStateRequest) -> Result<(), Error> {
        Ok(self
            .delete_bulk_state(Request::new(request))
            .await?
            .into_inner())
    }
}


#[async_trait]
pub trait DaprState: Sized {
    async fn get_state(&mut self, request: GetStateRequest) -> Result<GetStateResponse, Error>;
    async fn save_state(&mut self, request: SaveStateRequest) -> Result<(), Error>;
    async fn delete_state(&mut self, request: DeleteStateRequest) -> Result<(), Error>;
    async fn delete_bulk_state(&mut self, request: DeleteBulkStateRequest) -> Result<(), Error>;
}

/// A request for getting state
pub type GetStateRequest = dapr_v1::GetStateRequest;

/// A response from getting state
pub type GetStateResponse = dapr_v1::GetStateResponse;

/// A request for saving state
pub type SaveStateRequest = dapr_v1::SaveStateRequest;

/// A request for deleting state
pub type DeleteStateRequest = dapr_v1::DeleteStateRequest;

/// A request for deleting bulk state
pub type DeleteBulkStateRequest = dapr_v1::DeleteBulkStateRequest;

impl<K> From<(K, Vec<u8>)> for common_v1::StateItem
where
    K: Into<String>,
{
    fn from((key, value): (K, Vec<u8>)) -> Self {
        common_v1::StateItem {
            key: key.into(),
            value,
            ..Default::default()
        }
    }
}
