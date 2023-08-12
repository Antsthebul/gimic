use serde::Deserialize;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::io::Write;
use std::process;
use std::collections::VecDeque;
use std::fs;
use std::path;
use std::env;
use std::process::Command;
use std::ffi::OsStr;

// use crate::gimic::commands::*;

const TEMP_BASE_PATH: &str  = ".gump/tmp";
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        let result = 2+2;
        assert_eq!(result, 4)
    }
    #[test]
    fn assert_verify_gloc_exists(){
        
        let current_dir = path::PathBuf::from("/tmp/foo/bar/gloc.yaml");
        assert_eq!(current_dir.as_os_str(), verify_gloc_exists(&current_dir).unwrap().as_os_str());
       
    }

}

#[derive(Debug, Deserialize)]
pub struct BaseConfig{
    pub version: Option<u64>,
    pub repos: VecDeque<AlternateRepoConfig>,
   

}

#[derive(Debug, Deserialize)]
pub struct AlternateRepoConfig{
    pub name: Option<String>,
    pub alternate_repo: Option<String>,
    pub alternate_target: Option<String>,
    pub alternate_source: Option<String>,
    pub branch: Option<String>,
    pub cache: Option<String>,
    pub keep_repo: Option<bool>,
    pub overwrite_on_update: Option<bool>
}

impl BaseConfig{
    fn modify_defaults(&mut self){
        for repo in &mut self.repos{
            if repo.branch.is_none(){
                repo.branch = Some(String::from("main"))
            }
        }

    }

    pub fn build_mapping_w_yaml(config_path:path::PathBuf) -> BaseConfig{

        let config_file = fs::File::open(config_path).unwrap_or_else(|error|{
            panic!("Unable to open file: {:?}", error);
        });
    
        let mut config : BaseConfig = serde_yaml::from_reader(config_file).unwrap_or_else(|error|{
            panic!("Problem reading file: {:?}", error);
        }); 

        config.modify_defaults();
        config      
    }

    pub fn run_action(&self, action: &str, idx: u32, raw_args:VecDeque<String>){
        let _: Vec<&str> = raw_args.iter().map(|r| r.as_ref()).collect();
        match action {
            // "pull"=>self.pull(action),
            "checkout"=> self.checkout(action, idx),
            // "commit" => self.commit(action, idx, gargs),
            _ => println!("\n{:?} has not be implemented. But if you think it would be super helpful consider contributing!", action),
        }
    }
    fn commit(&self, action:&str, idx:u32, gargs:Vec<&str>){
        
        run_command(action, gargs);
    }

    fn checkout(&self, action:&str, idx:u32) {
        println!("\n\x1B[1m\x1b[33mWarning\x1B[0m Attempting to fetch alerternate repo");

        let repo: &AlternateRepoConfig;
        // if idx == 0{
        repo= self.repos.front().unwrap();
        // }
        // let base_path = path::PathBuf::from(TEMP_BASE_PATH).to_str().unwrap();
        
        let checkout_args = vec!["-b", &repo.branch.as_ref().unwrap().as_ref(),
             &repo.alternate_repo.as_ref().unwrap().as_ref(), TEMP_BASE_PATH];
        
        run_command("clone", checkout_args);
        
        let target_path = path::PathBuf::from(repo.alternate_target.as_ref().unwrap());

        
        let source_path:path::PathBuf =[TEMP_BASE_PATH, repo.alternate_source.as_ref().unwrap() ].iter().collect();
        
        if  target_path.exists() {
            fs::remove_file(&target_path).expect("Unable to remove target path")
        }


        fs::create_dir_all(target_path.parent().unwrap()).expect("Unable to create dir ");

        fs::copy(source_path,target_path).expect("Unable to copy to specified dir");

        // remove dirs because enitre repos will always be 'cloned'
        fs::remove_dir_all(TEMP_BASE_PATH).expect("Failed to remove");
        write_green("Success!", "Target has been updated");

    }

}


pub fn failed_to_locate_gloc(){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))
        .expect("Failed to set color");
    write!(&mut stdout, "\nFatal: ")
        .expect("Failed to write");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .expect("Failed to set color");
    writeln!(&mut stdout, "Unable to locate gloc.yaml!")
        .expect("Failed to right");
    
}

pub fn verify_gloc_exists(current_dir: &path::PathBuf) -> Result<path::PathBuf, &str>{
    let mut ancestors = current_dir.ancestors();
    while let Some(file) = ancestors.next(){
            for path in fs::read_dir(file).unwrap(){
                let path_res = path.unwrap();
                if path_res.file_name() == "gloc.yaml"{
                    return Ok(path_res.path())
                }
            }                
    }  
    failed_to_locate_gloc();
    Err("Not Found")
}   

pub fn create_temporary_file_store(current_dir: &path::PathBuf){
    let gump_dir_path:path::PathBuf = [&current_dir, path::Path::new(".gump")].iter().collect();
    
    match std::fs::create_dir(&gump_dir_path.as_os_str()){
        Err(error) => match error.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Unable to create .gump directory: {:?}", error)
            }
        _ =>()
    }
    
    let cache: path::PathBuf = [&gump_dir_path, path::Path::new("tmp")].iter().collect();

     match std::fs::create_dir(cache.as_os_str()){
        Err(error) =>  match error.kind(){
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Unable to create tmp/ within .gump: {:?}", error)
            }
        _=>()
    }             
} 

fn run_command(action:&str, args:Vec<&str>) {
    println!("Fetching repo..\n\n[GIT]");


    let mut command = Command::new("git");
    command
        .arg(action)
        .args(args.iter().map(|item|OsStr::new(item)))
        .status()
        .expect("Unable to pull down repo");
    }

fn write_green(ctext: &str, text: &str){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))
        .expect("Failed to set 'greren'");
    write!(&mut stdout,"\n{}",ctext)
        .expect("Failed to write");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .expect("Failed to set color");
    writeln!(&mut stdout, " {}", text)  
        .expect("Failed to write")
}