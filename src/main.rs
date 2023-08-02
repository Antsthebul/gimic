use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use serde_yaml;

use std::env;
use std::process::Command;
use std::path;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;

mod gimic;
use crate::gimic::*;

const TEMP_BASE_PATH: &str  = ".gump/tmp";
fn main() {
    let current_dir = env::current_dir().unwrap();
    let mut ancestors = current_dir.ancestors();
    let mut found = false;

    'outer: for file in ancestors.next(){
        for path in fs::read_dir(file).unwrap(){

            let name = path.unwrap().file_name();
            if name == "gloc.yaml" {
                found = true;
                break 'outer;
            }
        }
    }
    if !found{
        failed_to_locate_gloc();
    }

    let gump_dir_path:path::PathBuf = [&current_dir, path::Path::new(".gump")].iter().collect();
    
    if let Err(err) = std::fs::create_dir(&gump_dir_path.as_os_str()){
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Unable to create .gump directory: {:?}", err)
            }
    }

    let cache: path::PathBuf = [&gump_dir_path, path::Path::new("tmp")].iter().collect();

    if let Err(err) = std::fs::create_dir(cache.as_os_str()){
        match err.kind(){
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Unable to create tmp/ within .gump: {:?}", err)
            }
    }            
        
    

    println!("\n\x1B[1m\x1b[33mWarning\x1B[0m Attempting to fetch foreign repo");


    let config_file = fs::File::open("gloc.yaml").unwrap_or_else(|error|{
        panic!("Unable to open file: {:?}", error);
    });

    let mut config : BaseConfig = serde_yaml::from_reader(config_file).unwrap_or_else(|error|{
        panic!("Problem reading file: {:?}", error);
    });

    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: gimic [checkout|pull|status] [branch] <repository_url> <target_directory>\n");
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
        "checkout" => mimic_git_checkout(branch, config.foreign_repo,  config.foreign_target.unwrap()),
        "pull" => mimic_git_pull(config.foreign_repo, config.foreign_target.unwrap()),
        "status" => mimic_git_status(),
        _ => println!("Unknown action: {}", action),
    }
}

fn run_command(program: &str, action:&str, args: &[&Option<String>]) {
    println!("Fetching repo..\n\n[GIT]");

    let status = Command::new(program)
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

fn mimic_git_checkout(branch: Option<&String>, repository_url: Option<String>, target_directory: String) {
    
    run_command("git", "clone", &[ &repository_url, &Some(TEMP_BASE_PATH.to_owned())]);

    if let Some(branch_name) = branch {
        run_command("git","branch", &[&Some(branch_name.to_string())]);
    }
    
    let temp_path = format!("{}/tests/models.py", &TEMP_BASE_PATH);

    let output_directory: String = format!("{}", target_directory);
    println!("Output => {:?}", OsStr::new(&output_directory.to_owned()));

    std::fs::create_dir("test").expect("Unable to create dir");

    std::fs::copy(temp_path.to_owned(),output_directory).expect("Unable to copy to specified dir");

    // remove dirs because enitre repos will always be 'cloned'
    fs::remove_dir_all(TEMP_BASE_PATH).expect("Failed to remove");
    
    write_green();
}

fn write_green(){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    writeln!(&mut stdout, "Success!");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
}
fn mimic_git_pull(repository_url: Option<String>, target_directory: String) {
    mimic_git_checkout(None, repository_url, target_directory);
}

fn mimic_git_status(){
    run_command("git","status", &[]);
}