use std::fs;

use crate::{Args, handle_path_type::FileTypeTrait};


pub trait HandleArgs {
    /// Returns true if given argument is in Arguments vector;
    fn checkargs(&self,args:String) -> bool;
        
    ///Restrict a argument -> panics if given argument is in Arguemts  
    ///Example: if PathType != PathType::url {args.restrict("post")}
    fn restrict(&self,restriction:String);
}



impl HandleArgs for  Args{
    fn restrict(&self,restriction:String) {
        //Check type
        let path_type= &self.path_type();
        //Panic if restriction is found in argument
        if self.checkargs(restriction.clone()) {
            panic!("Path type: {:?}, Doesn not implement {restriction}", path_type);
        };
    }

    fn checkargs(&self ,args:String) -> bool {
        for arg in &self.args {
            if *arg == args {
                return true;
            }
        };
        return false;
    }

}

//This are the function that get called when a argument is found
pub trait ArgumentsFunctions {
    //Should only be called when path is a file:
    //Should be restricted from url path type:
    //Reads file and prints it to the screen
    fn read(&self) -> String;
}

impl ArgumentsFunctions for Args {
    fn read(&self) -> String {
        let data  = fs::read_to_string(&self.path);
        match data {
            Ok(content) => content,
            Err(err) => panic!("Coulnd't read the file error: {}", err)
        }
    }
}
