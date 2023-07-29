use std::env;
use std::process::Command;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: gimic [checkout|pull] [branch] <repository_url> <target_directory>");
        return;
    }

    let action = &args[1];
    let branch = if args.len() > 4 { Some(&args[2]) } else { None };
    let repository_url = &args[args.len() - 2];
    let target_directory = &args[args.len() - 1];

    match action.as_str() {
        "checkout" => mimic_git_checkout(branch, repository_url, target_directory),
        "pull" => mimic_git_pull(repository_url, target_directory),
        _ => println!("Unknown action: {}", action),
    }
}

fn run_command(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect("Failed to execute command");
    
    if !status.success() {
        println!("Command execution failed");
        std::process::exit(1);
    }
}

fn mimic_git_checkout(branch: Option<&String>, repository_url: &str, target_directory: &str) {
    let temp_dir = "temp_clone";

    run_command("git", &["clone", repository_url, temp_dir]);

    if let Some(branch_name) = branch {
        run_command("git", &["checkout", branch_name]);
    }

    if let Ok(entries) = fs::read_dir(target_directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                fs::remove_file(&path).or_else(|_| fs::remove_dir_all(&path)).ok();
            }
        }
    }

    run_command("cp", &["-r", &format!("{}/{}", temp_dir, target_directory), "."]);

    fs::remove_dir_all(temp_dir).ok();
}

fn mimic_git_pull(repository_url: &str, target_directory: &str) {
    mimic_git_checkout(None, repository_url, target_directory);
}
