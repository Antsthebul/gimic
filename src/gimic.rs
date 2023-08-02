use serde::Deserialize;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::io::Write;
use std::process;


#[cfg(test)]
mod tests{

    #[test]
    fn it_works(){
        let result = 2+2;
        assert_eq!(result, 4)
    }


}   
#[derive(Debug, Deserialize)]
pub struct BaseConfig{
    pub version: Option<u64>,
    pub foreign_repo: Option<String>,
    pub foreign_target: Option<String>,
    pub cache: Option<String>,
    pub keep_repo: Option<bool>
}

pub fn failed_to_locate_gloc(){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
    write!(&mut stdout, "\nFatal: ");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    writeln!(&mut stdout, "Unable to locate gloc.yaml!");
    process::exit(0)
}