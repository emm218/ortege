use self::proto::RegisterRequest;

pub mod proto {
    tonic::include_proto!("ortege");
}

#[derive(Default)]
pub struct AccountsService;

#[tonic::async_trait]
impl proto::accounts_server::Accounts for AccountsService {
    async fn register(
        &self,
        request: tonic::Request<RegisterRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
