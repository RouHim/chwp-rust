use std::process::Command;

pub fn execute_command(cmd: &String) -> String {
    let result = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&result.stdout.to_owned()).to_string();
    let stderr = String::from_utf8_lossy(&result.stderr.to_owned()).to_string();

    println!("{}", stdout);
    println!("{}", stderr);

    return [stdout, stderr].join("\n");
}