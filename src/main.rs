use std::{fs, io::Write};
use std::fs::File;
use serde_json::{Result, Value};
use tera::{Tera, Context};

fn generator(json: Value, ctx: &mut Context) -> Result<()> {

    if let Some(name) = json.get("name") {
        ctx.insert("name", name);
    } else {
        ctx.insert("name", "");
    }

    if let Some(phone) = json.get("phone") {
        ctx.insert("phone", phone);
    } else {
        ctx.insert("phone","");
    }

    if let Some(email) = json.get("email") {
        ctx.insert("email", email);
    } else {
        ctx.insert("email", "");
    }

    if let Some(linkedin) = json.get("linkedin") {
        ctx.insert("linkedin", linkedin);
    } else {
        ctx.insert("linkedin", "");
    }

    if let Some(github) = json.get("github") {
        ctx.insert("github", github);
    } else {
        ctx.insert("github", "");
    }
    

    if let Some(educations) = json.get("educations") {
        ctx.insert("educations", educations);
    } else {
        let edu: Vec<Value> = vec![];
        ctx.insert("educations", &edu);
    }

    if let Some(experiences) = json.get("experiences") {
        ctx.insert("experiences", experiences);
    } else {
        let exp: Vec<Value> = vec![];
        ctx.insert("experiences", &exp);
    }

    if let Some(projects) = json.get("projects") {
        ctx.insert("projects", projects);
    } else {
        let proj: Vec<Value> = vec![];
        ctx.insert("projects", &proj);
    }


    if let Some(languages) = json.get("skills").and_then(|nest| nest.get("languages")) {
        ctx.insert("languages", languages);
    }

    if let Some(frameworks) = json.get("skills").and_then(|nest| nest.get("frameworks")) {
        ctx.insert("frameworks", frameworks);
    }

    if let Some(tools) = json.get("skills").and_then(|nest| nest.get("tools")) {
        ctx.insert("tools", tools);
    }

    if let Some(libraries) = json.get("skills").and_then(|nest| nest.get("libraries")) {
        ctx.insert("libraries", libraries);
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
    let json_str: String= fs::read_to_string(file_path).expect("invalid json file path, or template.json file was moved");
    
    let json_data: Result<Value> = serde_json::from_str(&json_str);

    match json_data {
        Ok(json) => {
            match generator(json,&mut context) {
                Ok(_) => {
                    let rendered = tera.render("resume.tex", &context);

                    let tex_file_path = "src/output/jakes_resume.tex";
                    let mut tex_file = File::create(tex_file_path).unwrap();
                    let _ = tex_file.write_all(rendered.unwrap().as_bytes());

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