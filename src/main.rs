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
    
    pub args:Vec<String>,
}

fn request_to_string(response:Result<reqwest::blocking::Response, reqwest::Error>)->String{
    let request = match response {
            Ok(value)=> value,
            Err(err) => panic!("Error with request body, coulnd't read from body:  {}", err)
            
        };

        let request_text= match request.text() {
            Ok(value) => value,
            Err(err) => panic!("Couldn't convert repsonse to string: {:?}", err)
        };

        return request_text
}

fn make_request(type_request:&'static str, has_body:bool, args:Args){

    if type_request == "post" {
        let client = reqwest::blocking::Client::new();
        if !has_body {
        let request = client.post(&args.path)
            .send();
       
        let request_string = &request_to_string(request);

        print!("Response: {:?}", request_string);
        
        }

        if has_body {
        let request = client.post(&args.path)
            .json(&args.get_body())
            .send();
       
        let request_string = request_to_string(request);

        print!("Response: {:?}", request_string);
        }
    }

     else if type_request == "get" {
        let client = reqwest::blocking::Client::new();
        let request = client.get(args.path)
            .send();

        let request_string = request_to_string(request);

        print!("Response: {:?}", request_string);
    }

    else if type_request == "delete" {
        let client = reqwest::blocking::Client::new();
        if !has_body {
        let request = client.delete(&args.path)
            .send();
       
      
        let request_string =request_to_string(request);

        print!("Response: {:?}", request_string);
        
        }

        if has_body {
        let request = client.delete(&args.path)
            .json(&args.get_body())
            .send();
       
        let request_string = request_to_string(request);

        print!("Response: {:?}", request_string);
        }
    }
}

fn main(){
    let args = Args::parse();   


    if args.path_type() == PathType::Url{
    args.restrict("read".to_string());
    

    if args.checkargs("post".to_string()) {
        if args.checkargs("body".to_string()) {
            make_request("post",  true, args);
        }
        else {
            make_request("post",  false, args);
        }
    }

    else if args.checkargs("get".to_string()){
            args.restrict("body".to_string());
            make_request("get",  true, args);
    }

    else if args.checkargs("delete".to_string()){
        if args.checkargs("body".to_string()){
            make_request("delete",  true, args);
        }
        else {
            make_request("delete",true, args);
        }
       }
    }

   
    else if args.path_type() == PathType::File {
        args.restrict("post".to_string());
        args.restrict("get".to_string());
        args.restrict("delete".to_string());

        if args.checkargs("read".to_string()) {
            print!("{}", args.read());
        }

    }
}
