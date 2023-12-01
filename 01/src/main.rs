mod puzzle01;
mod puzzle02;

fn main() -> Result<(), std::io::Error> {
    puzzle01::main()?;
    puzzle02::main()?;

    Ok(())
}