// use common_dtos::commit_client::{
//     head_snapshot_request::HeadSnapshotRequest, types::account::account_dto::AccountAgDto,
// };
// use common_libs_account_api_client::AccountApiClient;
// use common_simple_types::ag_id::AgId;
// use tauri::{AppHandle, Manager, Runtime, State};

use tauri::{AppHandle, Runtime};

/// Access to the account-client APIs.
pub struct AccountClient<R: Runtime>(pub AppHandle<R>);

// impl<R: Runtime> AccountClient<R> {
//     // functions to expose Rust APIs to main app (Rust side)
//     pub async fn load(
//         &self,
//         access_token: &str,
//         acct_id: AgId,
//     ) -> Result<AccountAgDto, crate::Error> {
//         let account_api_client: State<'_, AccountApiClient> = self.0.state();
//         let payload = HeadSnapshotRequest::new(acct_id);
//         account_api_client
//             .get(access_token, payload)
//             .await
//             .map_err(|e| e.into())
//     }
// }
