use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Connect {
    command: String,
    args: Vec<String>,
    envs: HashMap<String, String>,
}

fn spawn_command(command: &Connect) -> anyhow::Result<()> {
    println!("executing command: {}", command.command);
    let start = std::time::Instant::now();
    let mut output = std::process::Command::new(&command.command)
        .args(&command.args)
        .envs(&command.envs)
        .spawn()?;

    match output.wait() {
        Ok(val) => {
            println!("Status :: {:?}, time: {:?}", val, start.elapsed());
        }
        Err(err) => {
            println!("Exit Error :: {:?}, time: {:?}", err, start.elapsed());
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
    let path = std::env::var("CONNECT_CONFIG")?;
    let commands: HashMap<String, Connect> = read_config(&path)?;
    commands.get(command).map(|value| spawn_command(value));
    Ok(())
}
