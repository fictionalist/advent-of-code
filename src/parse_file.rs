use std::{fs::File, io::Read};

fn separate_by_line(mut data: String) -> Vec<String> {
    let mut out : Vec<String> = Default::default();

    while let Some(idx) = data.find('\n') {
        let line = data[0..idx].to_string();
        out.push(line);
        data = data[(idx + 1)..].to_string();
    }

    out
}

pub fn read_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut fd = File::open(path)?;
    let mut buffer : String = String::new();
    
    if fd.read_to_string(&mut buffer)? == 0 {
        println!("`{}`: Empty file.", path);
        ()
    }

    let lines = separate_by_line(buffer);

    Ok(lines)
}