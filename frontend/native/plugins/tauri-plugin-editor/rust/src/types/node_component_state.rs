use std::sync::Arc;

use common_dtos::editor_client::wasm_components_code_get::{
    request::WasmComponentsCodeGetRequest, response::WasmComponentCodeGetStatus,
};
use common_libs_editor_api_client::EditorApiClient;
use common_nmg_core::db::NmgArc;
use common_wasm_components_store::ComponentStore;
use common_wasm_evaluators::node_info::{
    exports::cloudcad::node_info::description::{
        ValidateInsertRelationshipRequest, ValidateRelationshipsRequest,
    },
    NmgProxy, NodeInfoWasiView, WasmNodeInfo,
};
use frontend_tauri_plugins_common::types::bearer_tokens::BearerTokens;
use tauri::{AppHandle, Manager, Runtime};
use wasmtime::Store;

use crate::error::InternalApplicationError;

pub struct NodeComponentState {
    node_component_interface: Arc<WasmNodeInfo>,
    wasm_component_store: Arc<ComponentStore>,
}

impl NodeComponentState {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            node_component_interface: Arc::new(WasmNodeInfo::new()?),
            wasm_component_store: Arc::new(ComponentStore::new()?),
        })
    }

    pub(crate) fn component_store(&self) -> Arc<ComponentStore> {
        self.wasm_component_store.clone()
    }

    pub(crate) fn node_component_interface(&self) -> Arc<WasmNodeInfo> {
        self.node_component_interface.clone()
    }

    fn get_access_token<R: Runtime>(app: &AppHandle<R>) -> Result<String, crate::EditorError> {
        let bearer_tokens = app.state::<BearerTokens<R>>();

        bearer_tokens
            .access_token()
            .ok_or_else(|| InternalApplicationError::BearerTokensAccessTokenNotPresent.into())
    }

    pub(crate) async fn validate_properties<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        node_type: &str,
        properties: &str,
        nmg: NmgArc,
    ) -> Result<(), crate::EditorError> {
        self.load_component_codes(app, vec![node_type.to_owned()])
            .await?;
        let mut store = self.new_wasi_store(nmg);
        self.node_component_interface
            .validate_properties(
                node_type,
                properties,
                &self.wasm_component_store,
                &mut store,
            )
            .map_err(|e| crate::EditorError::RecoverableError(e.to_string()))
    }

    pub(crate) async fn validate_insert_relationship<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        node_type: &str,
        request: &ValidateInsertRelationshipRequest,
        nmg: NmgArc,
    ) -> Result<(), crate::EditorError> {
        self.load_component_codes(app, vec![node_type.to_owned()])
            .await?;
        let mut store = self.new_wasi_store(nmg);
        self.node_component_interface
            .validate_insert_relationship(
                node_type,
                request,
                &self.wasm_component_store,
                &mut store,
            )
            .map_err(|e| InternalApplicationError::Other(e).into())
    }

    pub(crate) async fn validate_relationships<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        node_type: &str,
        request: &[ValidateRelationshipsRequest],
        nmg: NmgArc,
    ) -> Result<(), crate::EditorError> {
        self.load_component_codes(app, vec![node_type.to_owned()])
            .await?;
        let mut store = self.new_wasi_store(nmg);
        self.node_component_interface
            .validate_relationships(node_type, request, &self.wasm_component_store, &mut store)
            .map_err(|e| InternalApplicationError::Other(e).into())
    }

    pub(crate) fn new_wasi_store(&self, nmg: NmgArc) -> Store<NodeInfoWasiView> {
        let nmg_proxy = NmgProxy::new(nmg);
        let node_info_wasi_view = NodeInfoWasiView::new(nmg_proxy);
        Store::new(self.wasm_component_store.engine(), node_info_wasi_view)
    }

    pub(crate) async fn load_component_codes<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        component_names: Vec<String>,
    ) -> Result<(), crate::EditorError> {
        let mut non_loaded_components_code = Vec::new();

        for component_name in component_names.iter() {
            if !self.component_store().has_component(component_name)? {
                non_loaded_components_code.push(component_name.to_owned());
            }
        }

        if non_loaded_components_code.is_empty() {
            return Ok(());
        }

        let editor_api_client = app.state::<EditorApiClient>();
        let access_token = Self::get_access_token(app)?;
        let payload = WasmComponentsCodeGetRequest {
            component_names: non_loaded_components_code.to_owned(),
        };
        let node_info_get_response = editor_api_client
            .get_wasm_components_code(&access_token, payload)
            .await?;

        match node_info_get_response.status {
            WasmComponentCodeGetStatus::Success { codes } => {
                for component_name in non_loaded_components_code.into_iter() {
                    let component_code = codes.get(&component_name).ok_or(
                        InternalApplicationError::NodeComponentCodeNotFound {
                            node_type: component_name.to_owned(),
                        },
                    )?;
                    self.component_store()
                        .register_component(component_name, component_code.clone())?;
                }
                Ok(())
            }
            WasmComponentCodeGetStatus::Failure(msg) => {
                Err(crate::EditorError::RecoverableError(msg))
            }
        }
    }
}
