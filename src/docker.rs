use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InspectData {
    #[serde(alias = "LowerDir")]
    lower_dir: String,
    #[serde(alias = "UpperDir")]
    upper_dir: String,
    #[serde(alias = "MergedDir")]
    merged_dir: String,
    #[serde(alias = "WorkDir")]
    work_dir: String,
}

pub fn inspect(container_name: &String) -> Vec<String> {
    let command = format!(
        "docker inspect --format='{{{{json .GraphDriver.Data}}}}' {}",
        &container_name
    );

    let output = utils::launch_command(&command);
    let inspect_data: InspectData = serde_json::from_str(&output).unwrap();
    let mut response: Vec<String> = Vec::new();

    response.push(inspect_data.merged_dir);
    response.push(inspect_data.upper_dir);
    response.push(inspect_data.work_dir);

    let splited = inspect_data.lower_dir.split(":");

    for item in splited {
        response.push(String::from(item));
    }

    response
}

#[derive(Serialize, Deserialize)]
pub struct ContainerInfo {
    #[serde(alias = "Names")]
    pub name: String,
}

pub fn get_containers() -> Vec<ContainerInfo> {
    let command = String::from("docker ps -a --format='{{json .}}|'");

    let output = utils::launch_command(&command);

    let mut containers_data: Vec<ContainerInfo> = Vec::new();
    let splitted = output.split("|");

    for item in splitted {
        if item.contains("{") {
            // detect if json is valid
            containers_data.push(serde_json::from_str(&item).unwrap());
        }
    }

    containers_data
}

pub fn get_volumes_size() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let command =
        String::from("du -s /var/lib/docker/overlay2/*/diff | sort -n -r | cut -d$'\t' -f2-");

    let output = utils::launch_command(&command);

    let splitted = output.split("\n");

    for line in splitted {
        result.push(String::from(line));
    }

    result
}
