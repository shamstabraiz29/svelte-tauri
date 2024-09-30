// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/nmg_from_branch_ag_dto.rs
use rustc_hash::FxHashMap;

use common_dtos::commit_client::types::branch::branch_dto::BranchAgDto;
use common_nmg_core::{
    db::{node::NodeBuilder, rel::RelBuilder, NmgArc},
    utilities::PropValue,
    ImmutableString,
};
use log::{debug, error, info};

use crate::error::NmgFromBranchAgDtoError;

pub(crate) fn populate_nmg_from_branch_ag_dto(
    nmg: NmgArc,
    branch_ag_dto: &BranchAgDto,
) -> Result<(), NmgFromBranchAgDtoError> {
    info!("Starting to populate NMG from BranchAgDto");
    debug!(
        "BranchAgDto contains {} nodes and {} relationships",
        branch_ag_dto.nodes.len(),
        branch_ag_dto.relationships.len()
    );

    let mut node_refs = FxHashMap::default();

    info!("Creating nodes");
    for (node_id, node) in &branch_ag_dto.nodes {
        debug!("Creating node with ID: {}", node_id);

        let mut labels: Vec<ImmutableString> = node
            .labels
            .iter()
            .map(|s| ImmutableString::from(s.as_str()))
            .collect();
        labels.push(ImmutableString::from(node.node_type.as_str()));
        labels.push(ImmutableString::from(node_id.to_string()));

        let properties: FxHashMap<String, PropValue> = node
            .properties
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone().into()))
            .collect();

        debug!(
            "Node {} has {} labels and {} properties",
            node_id,
            labels.len(),
            properties.len()
        );

        let node_builder = NodeBuilder::new(node.node_type.as_str())
            .with_node_id(node_id.to_string())
            .with_labels(labels)
            .with_properties(properties);

        match node_builder.create(nmg.clone()) {
            Ok(node_ref) => {
                debug!("Successfully created node with ID: {}", node_id);
                node_refs.insert(node_id.clone(), node_ref);
            }
            Err(e) => {
                error!("Failed to create node with ID: {}. Error: {:?}", node_id, e);
                return Err(NmgFromBranchAgDtoError::NmgOp(e));
            }
        }
    }

    info!("Successfully created {} nodes", node_refs.len());

    info!("Creating relationships");
    for (rel_id, rel) in &branch_ag_dto.relationships {
        debug!("Creating relationship with ID: {}", rel_id);

        let rel_type: ImmutableString = ImmutableString::from(rel.rel_type.clone());
        let mut labels: Vec<ImmutableString> = rel
            .labels
            .iter()
            .map(|s| ImmutableString::from(s.as_str()))
            .collect();
        labels.push(rel_type.clone());
        labels.push(ImmutableString::from(rel_id.to_string()));

        debug!(
            "Building a relationship of type {} from {} to {}",
            rel.rel_type, rel.from, rel.to
        );

        let from_node = node_refs.get(&rel.from).ok_or_else(|| {
            NmgFromBranchAgDtoError::RelFromNodeNotFound {
                rel_id: rel.id.clone(),
                node_id: rel.from.clone(),
            }
        })?;

        let to_node =
            node_refs
                .get(&rel.to)
                .ok_or_else(|| NmgFromBranchAgDtoError::RelToNodeNotFound {
                    rel_id: rel.id.clone(),
                    node_id: rel.to.clone(),
                })?;

        let properties: FxHashMap<String, PropValue> = rel
            .properties
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone().into()))
            .collect();

        debug!(
            "Relationship {} has {} labels and {} properties",
            rel_id,
            labels.len(),
            properties.len()
        );

        let rel_builder = RelBuilder::new(rel_type, from_node, to_node)
            .with_rel_id(rel_id.to_string())
            .with_properties(properties)
            .with_labels(labels);

        if let Err(e) = rel_builder.create(nmg.clone()) {
            error!(
                "Failed to create relationship with ID: {}. Error: {:?}",
                rel_id, e
            );
            return Err(NmgFromBranchAgDtoError::NmgOp(e));
        }
        debug!("Successfully created relationship with ID: {}", rel_id);
    }

    info!("Successfully populated NMG from BranchAgDto");
    Ok(())
}
