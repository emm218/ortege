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
use tonic::transport::Server;

mod accounts;

use accounts::{proto::accounts_server::AccountsServer, AccountsService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:5000".parse()?;
    let accounts_service = AccountsService;

    Server::builder()
        .add_service(AccountsServer::new(accounts_service))
        .serve(addr)
        .await?;

    Ok(())
}
