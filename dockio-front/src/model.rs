use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

// Drawio meta

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeKey(pub String, pub String);

impl serde::Serialize for NodeKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}-{}", self.0, self.1);
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for NodeKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 2 {
            // TODO return Err(_)
            Ok(NodeKey("".to_string(), "".to_string()))
        } else {
            Ok(NodeKey(parts[0].to_string(), parts[1].to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nodes(pub HashMap<NodeKey, DrawioNode>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawioNode {
    pub tname: String,
    pub cid: u16,
    pub server: String,
    pub r#type: DrawioNodeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawioNodeType {
    GitLabRepo,
    DockerContainer,
    SystemDService,
}

// implement FromStr for DrawioNodeType
impl std::str::FromStr for DrawioNodeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gitlab-repo" => Ok(DrawioNodeType::GitLabRepo),
            "docker-container" => Ok(DrawioNodeType::DockerContainer),
            "systemd-service" => Ok(DrawioNodeType::SystemDService),
            _ => Err(()),
        }
    }
}

// server actual status

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Orchestrator {
    SystemD,
    Docker,
}
