use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeKey(pub String, pub String);

#[derive(Debug, Clone)]
pub struct Nodes(pub HashMap<NodeKey, Node>);

#[derive(Debug, Clone)]
pub struct Node {
    pub x: i16,
    pub y: i16,
    pub value: String,
    pub cname: String,
    pub cid: u16,
    pub orchestrator: Orchestrator,
    pub server: String,
}

pub type Containers = Vec<Container>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub image: String,
    pub names: String,
    pub ports: String,
    #[serde(rename = "running_for")]
    pub running_for: String,
    pub size: String,
    pub state: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Orchestrator {
    SystemD,
    Docker,
}
