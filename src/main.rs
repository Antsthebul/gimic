use serde_yaml;
use serde::Deserialize;
use std::env;
use std::process::Command;
use std::path;
use std::ffi::OsStr;
use std::fs;

#[derive(Debug, Deserialize, Default)]
struct BaseConfig{
    version: Option<u64>,
    foreign_repo: Option<String>,
    foreign_target: Option<String>
}
fn main() {
    let current_dir  = env::current_dir().unwrap();
    let file_path: path::PathBuf = [&current_dir, &"gloc.yaml".into()].iter().collect();

    if !file_path.exists(){
        println!("\n\x1B[1m\x1B[31mError\x1B[0m Unable to find \"gloc.yaml\". Are you in the correct directory?");
        return;
       }

    let gump_dir_path:path::PathBuf = [current_dir, ".gump".into()].iter().collect();
    
    if let Err(err) = std::fs::create_dir(gump_dir_path.as_os_str()){
    
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Unable to create .gump directory: {:?}", err)
            }
    }

    println!("\n\x1B[1m\x1b[33mWarning\x1B[0m Attempting to fetch foreign repo");


    let config_file = fs::File::open("gloc.yaml").unwrap_or_else(|error|{
        panic!("Unable to open file: {:?}", error);
    });

    let mut config : BaseConfig = serde_yaml::from_reader(config_file).unwrap_or_else(|error|{
        panic!("Problem reading file: {:?}", error);
    });

    
    // // Find target directory and target Repo
    // if let Some(version) = config.foreign_target.get(&serde_yaml::Value::String("version".to_string())){
    //     match version {  
    //         serde_yaml::Value::Number(version) => println!("{:?}", version.as_u64().unwrap()),
    //         _ => println!("Version must be a number")
    //     }
    // }

    // if let Some(foreign_repo) = config.get(&serde_yaml::Value::String("foreign_repo".to_string())){
    //     match foreign_repo {  
    //         serde_yaml::Value::String(foreign_repo) => println!("{:?}", foreign_repo.as_str()),
    //         _ => println!("Version must be a number")
    //     }
    // }

    // if let Some(foreign_target) = config.get(&serde_yaml::Value::String("foreign_target".to_string())){
    //     match foreign_target {  
    //         serde_yaml::Value::String(foreign_target) => println!("{:?}", foreign_target.as_str()),
    //         _ => println!("Version must be a number")
    //     }
    // }

    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: gimic [checkout|pull] [branch] <repository_url> <target_directory>\n");
        // return;
    }

    let action = &args[1];
    let branch = if args.len() > 4 { Some(&args[2]) } else { None };
    if config.foreign_repo.is_none(){
        config.foreign_repo = Some(args[args.len() - 2].to_string())
    }

    if config.foreign_target.is_none() {
        config.foreign_target = Some(args[args.len() - 1].to_string())
    } 

    match action.as_str() {
        "checkout" => mimic_git_checkout(branch, config.foreign_repo, config.foreign_target),
        "pull" => mimic_git_pull(config.foreign_repo, config.foreign_target),
        _ => println!("Unknown action: {}", action),
    }
}

fn run_command(command: &str, action:&str, args: &[&Option<String>]) {
    println!("Fethcing repo");

    let status = Command::new(command)
        .arg(action)
        .args(args.iter().map(|item| {
            match item {
                Some(s) => OsStr::new(s),
                _=> OsStr::new("")
            } 
        }))
        .status()
        .expect("Failed to execute command");
    
    if !status.success() {
        println!("Command execution failed");
        std::process::exit(1);
    }
}

fn mimic_git_checkout(branch: Option<&String>, repository_url: Option<String>, target_directory: Option<String>) {
    let temp_dir = "temp_clone";

    run_command("git", "clone", &[ &repository_url, &target_directory]);

    if let Some(branch_name) = branch {
        run_command("git","branch", &[&Some(branch_name.to_string())]);
    }

    // if let Ok(entries) = fs::read_dir(target_directory) {
    //     for entry in entries {
    //         if let Ok(entry) = entry {
    //             let path = entry.path();
    //             fs::remove_file(&path).or_else(|_| fs::remove_dir_all(&path)).ok();
    //         }
    //     }
    // }

    // run_command("cp", &["-r", &format!("{}/{}", temp_dir, target_directory), "."]);

    // fs::remove_dir_all(temp_dir).ok();
}

fn mimic_git_pull(repository_url: Option<String>, target_directory: Option<String>) {
    mimic_git_checkout(None, repository_url, target_directory);
}
