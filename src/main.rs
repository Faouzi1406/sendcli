mod handle_path_type;
mod tests;
mod handle_args;

use clap::Parser;
use crate::{handle_args::{HandleArgs, ArgumentsFunctions}, handle_path_type::{FileTypeTrait, PathType}};

#[derive(Parser,Debug)]
#[command(about)]
pub struct Args {
    ///This is a path you could chose to read a file localy or a url: Example url: "https://.." or "http://.."; file:"./" || "/"  || "../"
    pub path:String,
    
    pub args:Vec<String>
}


fn main(){
    let args = Args::parse();   

    if args.path_type() == PathType::Url{
    args.restrict("read".to_string());
    let arg_path = &args.path;

    if args.checkargs("post".to_string()) {
        let client = reqwest::blocking::Client::new();
        let request = client.post(arg_path)
            .body("some body")
            .send();

        let request = match request {
            Some(value)=> value,
            Err(err) => panic!("Error with request body, coulnd't read from body:  {}", err)
            
        };
        print!("{:?}", request);
    }

    }

   
    if args.path_type() == PathType::File {
    args.restrict("post".to_string());
    args.restrict("get".to_string());
    if args.checkargs("read".to_string()) {
        print!("{}", args.read());
    }
    }


}
