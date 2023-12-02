mod parse_file;
mod day01;
mod day02;

fn main() -> Result<(), std::io::Error> {
    day01::main()?;
    day02::main()?;

    Ok(())
}