extern crate iron;
extern crate router;
extern crate markdown;
extern crate rustc_serialize;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::convert::From;
use rustc_serialize::{
    Decodable,
    Decoder
};

use iron::prelude::*;
use iron::status;
use iron::headers::{
    ContentType
};
use iron::middleware::Handler;
use iron::modifiers::Header;
use router::Router;
use rustc_serialize::json;
use std::env::args;

#[derive(RustcEncodable,RustcDecodable,Debug)]
struct Page
{
    title: String,
    location: String,
    url: String
}

fn index() {

}

fn get_config(path:&str) -> Result<Vec<Page>,Box<Error>> {
    let mut config = try!{File::open(path)};
    let structure = try!{json::Json::from_reader(&mut config)};
    Ok(try!{<Vec<Page>>::decode(&mut json::Decoder::new(structure))})
}

struct PageHandler {
    prefix: String
}
impl Handler for PageHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        //let ref path = req.extensions.get::<Router>().unwrap().find("path").unwrap_or("/");
        let config = get_config(&format!("{}/config.json",self.prefix)).unwrap();
        let mut path = String::new();
        for fragment in &req.url.path {
            path.push_str("/");
            path.push_str(&fragment);
        }
        println!("PATH ! {}",path);
        println!("{:?}",config);
        println!("{:?}",req);
        //    let mut headers = Headers::new();
        //    headers.set(ContentType::html());
        for page in config {
            if page.url == path {
                println!("MATCH!");
                return Ok(Response::with((status::Ok, markdown::file_to_html(Path::new(&format!("{}/{}",self.prefix,page.location))).unwrap(),Header(ContentType::html()))))
            }
        }
        println!("HELLO!");
        return Ok(Response::with(status::NotFound))

    }
}

fn main() {
    let prefix = args().nth(1).expect("error, content location not specified");
    Iron::new(PageHandler{prefix:prefix}).http("localhost:4000").unwrap();
}
