use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", about = "Remove a given substring from a string")]

struct Cli {
    input: String,
    remove: String,
}


fn remove_string(input: &str, target: &str) -> String {
    input.replace(target, "")
}

fn main() {
    let args = Cli::parse();

    let result = remove_string(&args.input, &args.remove);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_string() {
        assert_eq!(remove_string("hello world", "world"), "hello ");
        assert_eq!(remove_string("hello world", "hello"), " world");
        assert_eq!(remove_string("hello world", " "), "helloworld");
    }
}