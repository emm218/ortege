use ed25519::{Signature, SignatureBytes};
use ed25519_dalek::{Verifier, VerifyingKey};
use tonic::Status;

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
    ) -> Result<tonic::Response<()>, Status> {
        println!("REQUEST={request:?}");

        let request = request.into_inner();

        let mut request_body = request.username.to_owned().into_bytes();
        request_body.extend_from_slice(&request.identity);

        let identity = request
            .identity
            .try_into()
            .map_err(|_| Status::invalid_argument("identity is invalid"))?;
        let identity = VerifyingKey::from_bytes(&identity)
            .map_err(|_| Status::invalid_argument("identity is invalid"))?;

        let signature = request
            .signature
            .try_into()
            .map(|s: SignatureBytes| Signature::from_bytes(&s))
            .map_err(|_| Status::invalid_argument("signature is invalid"))?;

        identity
            .verify(&request_body, &signature)
            .map_err(|_| Status::invalid_argument("signature is invalid"))?;

        Ok(tonic::Response::new(()))
    }
}
