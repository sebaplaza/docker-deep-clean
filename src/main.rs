use std::fs;
mod docker;
mod utils;

fn main() {
    let all_volumes = docker::get_diff_volumes();
    let mut used_volumes: Vec<String> = Vec::new();

    let containers = docker::get_containers();
    for container in containers {
        println!("");
        println!("volumes for container {}", container.name);
        let volumes = docker::inspect(&container.name);
        for volume in volumes.clone() {
            println!("{}", &volume);
        }
        used_volumes.extend(volumes);
    }

    let mut unused_volumes: Vec<String> = Vec::new();

    for volume in all_volumes {
        let is_used = used_volumes.contains(&volume);
        if !is_used {
            unused_volumes.push(String::from(volume));
        }
    }

    // remove unused volumes
    for volume in unused_volumes {
        println!("");
        println!("to delete {}", volume);
        match fs::remove_dir_all(volume.clone()) {
            Ok(()) => println!("Successfully deleted the folder {}", volume),
            Err(e) => println!("Error deleting the folder: {} {}", volume, e),
        };
    }
}
