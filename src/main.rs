
use std::collections::VecDeque;
use std::env;
use std::process;

mod gimic{
    pub mod gimic;
}

use crate::gimic::gimic::*;


fn main() {
    let mut args: VecDeque<String> = env::args().collect();

    let current_dir = env::current_dir().unwrap();
    // TODO: Allow settings for verbosity
    let config: BaseConfig;
    if args.len() < 3{

        config = match verify_gloc_exists(&current_dir){
                Ok(x) => BaseConfig::build_mapping_w_yaml(x),
                _ =>process::exit(1)
        };
        
        create_temporary_file_store(&current_dir);  
        args.pop_front(); // drop program name
        let action = args.pop_front().unwrap();
        if args.contains(&"skip-worktree".to_string()){
            let target = args.pop_front().unwrap();
            skip_worktree(target)
        }else{

            let idx: u32 = 0;
            config.run_action(action.as_ref(), idx, args);
        }

    }
}
