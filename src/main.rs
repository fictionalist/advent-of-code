mod parse_file;
mod day01;
mod day02;
mod day03;

fn main() -> Result<(), std::io::Error> {
    day01::main()?;
    day02::main()?;
    day03::main()?;

    Ok(())
}