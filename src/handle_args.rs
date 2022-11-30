use std::{collections::HashMap, fs};

use crate::{handle_path_type::FileTypeTrait, Args};

pub trait HandleArgs {
    /// Returns true if given argument is in Arguments vector;
    fn checkargs(&self, args: String) -> bool;

    ///Restrict a argument -> panics if given argument is in Arguemts  
    ///Example: if PathType != PathType::url {args.restrict("post")}
    fn restrict(&self, restriction: String);

    //Get body return the body given by user in Vec<string> for a post or other request that can
    //have a body
    fn get_body(&self) -> HashMap<String, String>;
}

pub trait FindFunctions {
    fn get_index(&self, needle: String) -> usize;
}

impl FindFunctions for Vec<String> {
    fn get_index(&self, needle: String) -> usize {
        let mut i: usize = 0;

        loop {
            if i > self.len() {
                return 0;
            }
            if self[i] == needle {
                i += 1;
                break;
            }
            i += 1;
        }

        i
    }
}

impl HandleArgs for Args {
    fn restrict(&self, restriction: String) {
        //Check type
        let path_type = &self.path_type();
        //Panic if restriction is found in argument
        if self.checkargs(restriction.clone()) {
            panic!(
                "Path type: {:?}, Doesn not implement {restriction}",
                path_type
            );
        };
    }

    fn get_body(&self) -> HashMap<String, String> {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        let start_body = self.args.get_index("startBody".to_string());
        let end_body = self.args.get_index("endBody".to_string());

        if start_body == 0 || end_body == 0 {
            panic!("Body argument was given but startbody and or endBody is missing");
        }

        for value in self.args[start_body..end_body].iter() {
            if value.contains(":") {
                let (value_a, value_b) = value.split_once(':').unwrap();

                hash_map.insert(value_a.to_string(), value_b.to_string());
            }
        }

        println!("body: {:?}", hash_map);

        hash_map
    }

    fn checkargs(&self, args: String) -> bool {
        for arg in &self.args {
            if *arg == args {
                return true;
            }
        }
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
        let data = fs::read_to_string(&self.path);
        match data {
            Ok(content) => content,
            Err(err) => panic!("Coulnd't read the file error: {}", err),
        }
    }
}
