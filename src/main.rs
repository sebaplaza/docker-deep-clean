use std::collections::HashSet;

mod docker;
mod utils;

fn main() {
    let containers = docker::get_containers();
    let all_volumes = docker::get_volumes_size();
    let mut used_volumes: Vec<String> = Vec::new();

    for container in containers {
        println!("");
        println!("volumes for container {}", container.name);
        let volumes = docker::inspect(&container.name);
        for volume in volumes.clone() {
            println!("{}", &volume);
        }
        used_volumes.extend(volumes);
    }

    let set1: HashSet<String> = HashSet::from_iter(all_volumes);
    let set2: HashSet<String> = HashSet::from_iter(used_volumes);

    let volumes_to_delete: Vec<&String> = set2.difference(&set1).collect();

    for volume in volumes_to_delete {
        print!("");
        println!("to delete {}", volume);
        let command = format!("rm -rf {}", volume);
        utils::launch_command(&command);
        println!("deleted {}", volume);
    }
}
