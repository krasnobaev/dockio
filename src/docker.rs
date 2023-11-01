use std::process::Command;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Labels")]
    pub labels: String,
    #[serde(rename = "LocalVolumes")]
    pub local_volumes: String,
    #[serde(rename = "Mounts")]
    pub mounts: String,
    #[serde(rename = "Names")]
    pub names: String,
    #[serde(rename = "Networks")]
    pub networks: String,
    #[serde(rename = "Ports")]
    pub ports: String,
    #[serde(rename = "RunningFor")]
    pub running_for: String,
    #[serde(rename = "Size")]
    pub size: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerFront {
    pub image: String,
    pub names: String,
    pub ports: String,
    pub running_for: String,
    pub size: String,
    pub state: String,
    pub status: String,
}

impl From<&Container> for ContainerFront {
    fn from(item: &Container) -> Self {
        ContainerFront {
            image: item.image.clone(),
            names: item.names.clone(),
            ports: item.ports.clone(),
            running_for: item.running_for.clone(),
            size: item.size.clone(),
            state: item.state.clone(),
            status: item.status.clone(),
        }
    }
}

impl Into<ContainerFront> for Container {
    fn into(self) -> ContainerFront {
        ContainerFront {
            image: self.image.clone(),
            names: self.names.clone(),
            ports: self.ports.clone(),
            running_for: self.running_for.clone(),
            size: self.size.clone(),
            state: self.state.clone(),
            status: self.status.clone(),
        }
    }
}

pub fn get_containers() -> Vec<ContainerFront> {
    // docker ps --format json
    let output = Command::new("docker")
        .arg("ps")
        .arg("--no-trunc")
        .arg("--format")
        // .arg("json") // Docker 21+
        .arg("{{json .}}") // Docker 20 workaround
        .output()
        .expect("Failed to execute command");

    let str = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let containers = str.lines().map(|row| {
        // log::debug!("row: {}", row);
        let deserialized: Container = serde_json::from_str(&row).unwrap();
        deserialized
    }).collect::<Vec<_>>();
    let f_containers: Vec<ContainerFront> = containers.iter().map(|c| {
        c.into()
    }).collect::<Vec<_>>();

    f_containers
}
