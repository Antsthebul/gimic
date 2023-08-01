use serde::Deserialize;


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
    pub cache: Option<String>
}

