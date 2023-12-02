use std::{fs::File, io::Read};

fn separate_by_line(data: String) -> Vec<String> {
    let mut out : Vec<String> = Default::default();
    let mut lines = data.lines();

    while let Some(line) = lines.next() {
        out.push(line.to_string());
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