use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;
use std::fs;
use std::io::{BufReader, BufWriter, Result, Write};
use std::path::PathBuf;

fn add_activate_alias(aliases: &Vec<Alias>, alias_file: &mut BufWriter<fs::File>) -> Result<()> {
    write!(alias_file, "function rustivate() {{\n")?;
    for alias in aliases {
        let alias_command = format!("    alias {}='{}'\n", alias.name, alias.value);
        // println!("{}", alias_command);
        write!(alias_file, "{}", alias_command)?;
    }
    write!(alias_file, "}}\n\n")?;
    Ok(())
}

fn add_deactivate_alias(aliases: &Vec<Alias>, alias_file: &mut BufWriter<fs::File>) -> Result<()> {
    write!(alias_file, "function derustivate() {{\n")?;
    for alias in aliases {
        let alias_command = format!("    unalias {}\n", alias.name);
        // println!("{}", alias_command);
        write!(alias_file, "{}", alias_command)?;
    }
    write!(alias_file, "}}\n\n")?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Alias {
    name: String,
    value: String,
}

fn read_aliases() -> Result<Vec<Alias>> {
    let rustivate_home = get_rustivate_home();
    let user_settings_path = rustivate_home.join("user_settings.json");
    let json_path = if user_settings_path.exists() {
        user_settings_path
    } else {
        PathBuf::from("resources/default.json")
    };
    let file = fs::File::open(json_path)?;
    let reader = BufReader::new(file);
    let aliases: Vec<Alias> = serde_json::from_reader(reader)?;
    Ok(aliases)
}

fn get_rustivate_home() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.join(".rustivate")
}

fn get_alias_sh() -> PathBuf {
    let rustivate_home = get_rustivate_home();
    rustivate_home.join("aliases.sh")
}

fn install_rust_tools() {
    let rust_tools = ["bat", "exa", "fd-find", "ripgrep", "du-dust"];
    for &tool in rust_tools.iter() {
        println!("Start install {}", tool);
        let output = Command::new("cargo").arg("install").arg(tool).output().expect("");
        let output = std::str::from_utf8(&output.stderr).unwrap();
        println!("{}", output);
    }
}

fn main() -> Result<()> {
    let alias_script = get_alias_sh();
    fs::create_dir(get_rustivate_home());
    let mut f = BufWriter::new(fs::File::create(alias_script).unwrap());

    let aliases = read_aliases()?;
    add_activate_alias(&aliases, &mut f)?;
    add_deactivate_alias(&aliases, &mut f)?;
    f.flush()?;
    println!("End set alias.");
    install_rust_tools();
    println!("End tools installation.");
    Ok(())
}
