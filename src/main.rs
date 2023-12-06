mod parse_file;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() -> Result<(), std::io::Error> {
    day01::main()?;
    day02::main()?;
    day03::main()?;
    day04::main()?;
    day05::main()?;

    Ok(())
}