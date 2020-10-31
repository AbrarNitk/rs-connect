use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Connect {
    command: String,
    args: Vec<String>,
    envs: HashMap<String, String>,
}

fn spawn_command(command: &Connect) -> anyhow::Result<()> {
    println!("executing command");
    let mut output = std::process::Command::new(&command.command)
        .args(&command.args)
        .envs(&command.envs)
        .spawn()?;

    match output.wait() {
        Ok(val) => {
            println!("Status :: {:?}", val);
        }
        Err(err) => {
            println!("Exit Error :: {:?}", err);
        }
    };
    Ok(())
}

pub fn read_config<T: serde::de::DeserializeOwned>(file_name: &str) -> anyhow::Result<T> {
    let exists = std::path::Path::new(file_name).exists();
    if !exists {
        return Err(anyhow::anyhow!("config file did not exists: {}", file_name));
    }
    Ok(serde_json::from_reader(std::fs::File::open(
        file_name.clone(),
    )?)?)
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).unwrap();
    println!("Hello, world!: {:?}", args);
    let connect: HashMap<String, Connect> = read_config("command.json")?;
    connect.get(command).map(|value| spawn_command(value));
    Ok(())
}
