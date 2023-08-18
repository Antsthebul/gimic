use chrono::Local;
use regex::Regex;
use serde::Deserialize;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


use std::io::Write;
use std::io;
use std::collections::VecDeque;
use std::fs;
use std::path;
use std::error::Error;
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

    #[test]
    fn test_traverse_dir_returns_1(){
        let current = path::PathBuf::from("gloc.yaml");
        let mut entries: Vec<path::PathBuf> = vec![];
        visit_dirs(current.as_path(),&mut  entries);

        assert_eq!(OsStr::new("gloc.yaml"), entries.pop().unwrap().as_os_str())
    }

}

#[derive(Debug, Deserialize)]
pub struct BaseConfig{
    pub version: Option<u64>,
    pub repos: VecDeque<AlternateRepoConfig>,
    pub fail_fast: Option<bool>
   

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
                repo.branch = Some(String::from("main"));
            }
            if repo.name.is_none(){
                let re = Regex::new(r"/(?<name>[^/]+)\.git").unwrap();
                let base_name = match  re.captures(repo.alternate_repo.as_ref().unwrap()){
                    Some(name) => name["name"].to_owned(),
                    _=> String::from("tmp"),
                };
                println!("{}", base_name);
                let cur_time = Local::now().format("%m%d%y_%H%M%S");
                let new_name = format!("{}_{}",base_name, cur_time );
                repo.name = Some(new_name)
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

        println!("\n\x1B[1m\x1b[33mWarning\x1B[0m Attempting to fetch alerternate repo\n");

        let repo: &AlternateRepoConfig;
        // if idx == 0{
        repo= &self.repos[idx.try_into().unwrap()];
        // }
        // let base_path = path::PathBuf::from(TEMP_BASE_PATH).to_str().unwrap();
        let mut repo_path = path::PathBuf::from(TEMP_BASE_PATH);
        repo_path.push(path::PathBuf::from(repo.name.as_ref().unwrap()));

        let checkout_args = vec!["-b", &repo.branch.as_ref().unwrap().as_ref(),
             &repo.alternate_repo.as_ref().unwrap().as_ref(), &repo_path.as_os_str().to_str().unwrap()];
    
        let target_path = path::PathBuf::from(repo.alternate_target.as_ref().unwrap());


        let source_path:path::PathBuf =[repo_path.as_os_str().to_str().unwrap(), repo.alternate_source.as_ref().unwrap() ].iter().collect();

        let mut response = String::new();

        if target_path.exists(){
            
            println!("We found that {:?} already exists. Do you want to overwrite? [y/N]: ", target_path);
            io::stdin()
            .read_line(&mut response)
            .expect("Error on input");

            let responses = vec!["y", "yes"];
            if !responses.contains(&response.trim().to_lowercase().as_str()){
                println!("Exiting...");
                
                std::process::exit(0);
            }
        
            
    
        }
        run_command("clone", checkout_args);
        
        

        let fail_fast = true;
        let file_list = traverse_dir(&source_path);

        copy_files(file_list,target_path.to_owned(), fail_fast);
        
        if !repo.keep_repo.unwrap(){
            fs::remove_dir_all(target_path).expect("Failed to remove");
        } 

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

fn write_styled(color_choice: &str, ctext:&str, text:&str) -> Result<(), Box<dyn Error>>{
    let color = match color_choice {
        "green" => Color::Green,
        "red" => Color::Red,
        _=> Color::White
    };

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
    write!(&mut stdout,"\n{}",ctext)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(&mut stdout, " {}", text)?;
    Ok(())
}

/// Calls `visit_dir`, which is that acutal traversal utility.
///  A Vec<path::PathBuf> is returned
fn traverse_dir(current:&path::PathBuf) -> Vec<path::PathBuf>{
    let mut entries: Vec<path::PathBuf> = vec![];
    visit_dirs(current.as_path(),&mut  entries);
    entries
}

/// This is the main logic to decide whether the `target` becomes a
/// directory of file. `file_list` contains absolute paths, starting at either 
/// root dir or where gloc.yaml lives, gloc.yaml taking precendence.
/// A message is generated if (un)successful
fn copy_files(file_list:Vec<path::PathBuf>, to:path::PathBuf, fail_fast:bool){

    let length = file_list.len();
    let mut fail = false;    

    for file in file_list{ //  list of absolute files (may be dir contents)
        // println!("Copying file {:?}...", file); // turn on with verbosity
        // find root of command(point of refernce) if not using gloc.yaml, + file
        // .gump/tmp will always be root wehter CLI or gloc.yaml
        let clean_source = file.strip_prefix(TEMP_BASE_PATH).unwrap();
        
        // strip the base parts beause either file OR cotnents of dir
        let mut comps:VecDeque<&OsStr> = clean_source.components().map(|comp| comp.as_os_str()).collect();
        
        if length == 1{
            // then target will be file
            
            let mut base_to:VecDeque<&OsStr>  = to.components().map(|r| r.as_os_str()).collect();
            base_to.pop_back(); // drop file_name to create_dirs

            let dirs_to_create: path::PathBuf = base_to.iter().map(|r| path::PathBuf::from(r)).collect();
            fs::create_dir_all(path::PathBuf::from(dirs_to_create))
            .expect("Unable to create dir");

            let res = match fs::copy(&file, &to) {           
                Err(err)=> Err(err),
                Ok(_)=> Ok(())
            };

            if fail_fast && res.is_err(){
                let mut text = String::from("Unable to copy to specified dir: ");
                text.push_str(format!("{:?}", res).as_str());

                write_styled("red","Fatal!",&text)
                .expect("Couldnt write Style");
                fail = true;
                break;
            }            
        }else{
            // target is dir, we dont need to remove base becuase this will contain all conetnts
        
            comps.pop_front(); // Anything after pop dir needs to be nested into target
            let new_target = to.clone().join::<path::PathBuf>(comps.iter().map(|r| path::PathBuf::from(r)).collect()); 
   
            // Now we still need to pop base of nwe targets
            let mut base_to:VecDeque<&OsStr>  = new_target.components().map(|r| r.as_os_str()).collect();
            base_to.pop_back();

            let dirs_to_create: path::PathBuf = base_to.iter().map(|r| path::PathBuf::from(r)).collect();
            fs::create_dir_all(path::PathBuf::from(dirs_to_create))
            .expect("Unable to create dir");

            

            let res = match fs::copy(&file, &new_target) {           
                Err(err)=> Err(err),
                Ok(_)=> Ok(())
            };
            if fail_fast && res.is_err(){
                let mut text = String::from("Unable to copy to specified dir: ");
                text.push_str(format!("{:?}", res).as_str());

                write_styled("red","Fatal!",&text)
                .expect("Couldnt write Style");

                break;
            }
        }
        if !fail{
            let text = format!(" Copied {} file(s)", length);
            write_styled("green", "Success!", &text)
                .expect("Failed to write");
        }

    }

    // default operation is to remove (which will conflict with commit)

 
}

fn visit_dirs(dir: &path::Path, entries: &mut Vec<path::PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, entries)?;
            } else {

                entries.push(path.into());
            }
        }
    }else{
        entries.push(dir.into());        
    }
    Ok(())
}