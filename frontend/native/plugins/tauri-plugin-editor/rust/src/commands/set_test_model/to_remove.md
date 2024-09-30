```rust
// Generate the derived model item ID for the ALB to ASG relationship viewport item
// Get the relationship participants
let alb_id = id_map.get("alb_id").unwrap();
let listener_id = id_map.get("listener_id").unwrap();
let target_group_id = id_map.get("target_group_id").unwrap();
let asg_id = id_map.get("asg_id").unwrap();
let mut hasher = std::hash::DefaultHasher::new();
alb_id.hash(&mut hasher);
listener_id.hash(&mut hasher);
target_group_id.hash(&mut hasher);
asg_id.hash(&mut hasher);
let derived_model_item_id = format!("{:0>26}", hasher.finish()).to_ascii_uppercase();

source_content = source_content.replace("{alb_to_asg_derived_id}", &derived_model_item_id);
```

```json
{
    "ViewportItem": {
        "Create": {
            "branch_id": "{branch_id}",
            "viewport_item_id": "<alb_to_asg_viewport_item_id: generate_ag_id>",
            "model_item_id": "{alb_to_asg_derived_id}",
            "viewport_id": "{viewport_id}",
            "properties": {
                "source": "{alb_id}",
                "target": "{asg_id}",
                "edge_type": "straight"
            }
        }
    }
}
```
