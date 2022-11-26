use crate::Args;
//Trait for given args parsed by clap


//This can either be a type of file or type of url 
//Or return and error based on value
#[derive(Debug, PartialEq)]
pub enum PathType{
    File,
    Url,
    Error
}


pub trait FileTypeTrait {
    fn first_char(&self) -> char;
    fn type_is_file(&self, first_char:char)  -> bool;
    fn type_is_url(&self, first_char:char)  -> bool;
    fn path_type(&self) -> PathType;
}
    
impl FileTypeTrait for Args{
    fn first_char(&self) -> char{
        let path_or_url = self.path.clone();
        path_or_url.chars().next().unwrap()
    }

    //When user wan'ts to give a file type it has to either start with '/', '.' or '/'
    fn type_is_file(&self, first_char:char)  -> bool{
        match  first_char{
            '.' => true, 
            '~' => true,
            '/' => true,
            _ => false
        }
    }
    
    fn type_is_url(&self, first_char:char)  -> bool{
        match  first_char{
            'h' => true, 
            _ => false
        }
    }

    ///Returns the path type general
    fn path_type(&self) -> PathType{
        if self.type_is_url(self.first_char()) {return PathType::Url};
        if self.type_is_file(self.first_char()) {return PathType::File};

        PathType::Error
    }

       
}


