use std::error::Error;
use std::path::Path;
use std::{env, fs};

fn get_all_proto(dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut protofiles: Vec<String> = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map(|e| e == "proto").unwrap_or(false) {
                let proto = fs::canonicalize(entry.path()).unwrap();
                protofiles.push(proto.to_str().unwrap().to_string());
            }
        }
        println!("protofiles to comilpe: {:?}", protofiles);
        return Ok(protofiles);
    }
    Err("Not a directory".into())
}
fn main() -> Result<(), Box<dyn Error>> {
    let path_prefix = "../../protobufs";
    let curr_dir = env::current_dir().unwrap();
    let full_path = curr_dir.join(path_prefix);
    let src = fs::canonicalize(full_path.clone()).unwrap();
    println!("OUT_DIR: {}", env::var("OUT_DIR").unwrap());
    let protobufs = get_all_proto(&full_path)?;
    prost_build::compile_protos(&protobufs, &[&src.to_str().unwrap()])?;
    Ok(())
}
