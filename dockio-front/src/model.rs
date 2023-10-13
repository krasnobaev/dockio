use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Nodes(pub HashMap<String, Node>);

#[derive(Debug, Clone)]
pub struct Node {
    pub x: u16,
    pub y: u16,
    pub value: String,
    pub cname: String,
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
