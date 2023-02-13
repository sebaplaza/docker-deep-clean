use std::process::Command;

pub fn launch_command(command: &String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    let output = match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(err) => panic!("Problem parsing command output: {:?}", err),
    };

    output
}
