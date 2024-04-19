/*
 * Copyright (C) 2024 Emmy Emmycelium
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Affero General Public License as published by the Free Software Foundation, either version 3
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along with this
 * program.  If not, see <https://www.gnu.org/licenses/>.
 */
mod proto {
    tonic::include_proto!("ortege");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = proto::accounts_client::AccountsClient::connect("http://[::1]:5000").await?;

    let request = proto::RegisterRequest {
        username: "emmy".to_string(),
        identity: vec![8],
        signature: vec![9],
    };

    let response = client.register(request).await?;

    println!("RESPONSE={response:?}");

    Ok(())
}
