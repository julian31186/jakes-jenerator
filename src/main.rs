use std::fs;
use serde_json::{Result, Value};
use tera::{Tera, Context};
mod schema;

fn generator(json: Value, mut ctx: Context) -> Result<()> {

    //Return error upon any missing fields
    
    ctx.insert("name", &json["name"]);
    ctx.insert("phone", &json["phone"]);
    ctx.insert("email", &json["email"]);
    ctx.insert("linkedin", &json["linkedin"]);
    ctx.insert("github", &json["github"]);

    //check for valid array here**
    for edu in json["education"].as_array().unwrap() {
        let edu: Result<schema::Education> = serde_json::from_value(edu.clone());
        match edu {
            Ok(edu) => {
                println!("Name: {}\nDegree: {}\n", edu.school, edu.degree);
            }
            Err(e) => {
                println!("Unable to parse education -> {}", e);
            }
        }
    }

    Ok(())
}

fn generate_resume() -> Result<()> {

    let tera = match Tera::new("templates/**/resume.tex") {
        Ok (t) => t,
        Err (e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new(); 
    
    let file_path: &str = "src/input/resume.json";
    let json_str: String= fs::read_to_string(file_path).expect("invalid json file path, or resume.json file was moved");
    
    let json_data: Result<Value> = serde_json::from_str(&json_str);

    match json_data {
        Ok(json) => {
            match generator(json,context) {
                Ok(_) => {
                    println!("Resume successfully created!");
                }
                Err(e) => { 
                    println!("Unable to create resume -> {}", e); 
                }
            }

            println!("JSON file parsed successfully!");
        },
        Err (e) => {
            println!("{}", e);
        }
    }

    Ok(())
}

fn main() {
    let _  = generate_resume();
}