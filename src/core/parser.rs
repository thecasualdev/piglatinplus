pub fn parse() -> Result<super::super::Args, String> {

    let mut args = std::env::args().skip(1).peekable();

    let mut verbose = false;
    let mut output = None;
    let mut input  = None;

    while let Some(arg) = args.next() {
        
        match arg.as_str() {
            "-V" | "--verbose" => verbose = true,
            "-O" | "--output" => output = Some(get_next_value(&mut args, arg.as_str())?),
            "-I" | "--input" => input = Some(get_next_value(&mut args, arg.as_str())?),

            _ => println!("No valid flag provided or has been provided incorrectly!")
        }

    }

    Ok(super::super::Args {
        verbose,
        output,
        input
    })

}

fn get_next_value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    args.next().ok_or(format!("Missing value for {}", flag))
}