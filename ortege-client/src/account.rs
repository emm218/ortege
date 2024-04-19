use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use rand::thread_rng;
use tonic::codegen::StdError;

mod proto {
    tonic::include_proto!("ortege");
}

pub struct Account {
    username: String,
    identity: SigningKey,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Grpc(#[from] tonic::Status),
}

impl Account {
    pub fn new<T: ToString>(username: T) -> Self {
        Self {
            username: username.to_string(),
            identity: SigningKey::generate(&mut thread_rng()),
        }
    }

    pub async fn register<D>(&self, dst: D) -> Result<(), Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let mut client = proto::accounts_client::AccountsClient::connect(dst).await?;

        let mut request_body = self.username.to_owned().into_bytes();
        request_body.extend_from_slice(self.identity.verifying_key().as_bytes());

        let signature = self.identity.sign(&request_body).to_vec();

        let identity = request_body.split_off(self.username.len());
        // we just made the bytes from a string
        let username = unsafe { String::from_utf8_unchecked(request_body) };

        let request = proto::RegisterRequest {
            username,
            identity,
            signature,
        };

        let response = client.register(request).await?;

        println!("RESPONSE={response:?}");

        Ok(())
    }
}
