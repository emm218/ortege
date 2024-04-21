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
use tonic::transport::Channel;
use tonic::transport::Endpoint;

use crate::account::proto::accounts_client::AccountsClient;

mod proto {
    tonic::include_proto!("ortege");
}

#[derive(Debug, Clone)]
pub struct RegistrationSession {
    identity: SigningKey,
    client: AccountsClient<Channel>,
}

#[derive(Debug, Clone)]
pub struct Account {
    username: String,
    identity: SigningKey,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("username already taken")]
    UsernameTaken,
    #[error(transparent)]
    Other(tonic::Status),
}

impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        match value.code() {
            tonic::Code::AlreadyExists => Error::UsernameTaken,
            _ => Error::Other(value),
        }
    }
}

impl RegistrationSession {
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<Endpoint>,
        D::Error: Into<StdError>,
    {
        Ok(Self {
            client: AccountsClient::connect(dst).await?,
            identity: SigningKey::generate(&mut thread_rng()),
        })
    }

    pub async fn register<T: ToString>(mut self, username: T) -> Result<Account, (Error, Self)> {
        let username = username.to_string();
        let username_len = username.len();
        let mut request_body = username.into_bytes();
        request_body.extend_from_slice(self.identity.verifying_key().as_bytes());

        let signature = self.identity.sign(&request_body).to_vec();

        let identity = request_body.split_off(username_len);
        // we just made the bytes from a string
        let username = unsafe { String::from_utf8_unchecked(request_body) };

        let request = proto::RegisterRequest {
            username: username.clone(),
            identity,
            signature,
        };

        match self.client.register(request).await {
            Ok(response) => {
                println!("RESPONSE={response:?}");

                Ok(Account {
                    username,
                    identity: self.identity,
                })
            }
            Err(e) => Err((e.into(), self)),
        }
    }
}
