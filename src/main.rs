mod parse_file;
mod day011;
mod day012;

fn main() -> Result<(), std::io::Error> {
    day011::main()?;
    day012::main()?;

    Ok(())
}