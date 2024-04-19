/*
 * Copyright (C) 2024 Emmy Emmycelium
 *
 * This file is part of ortege
 *
 * Ortege is free software: you can redistribute it and/or modify it under the terms of the GNU
 * Affero General Public License as published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along with this program.
 * If not, see <https://www.gnu.org/licenses/>.
 */
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
