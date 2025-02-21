use clap::{Parser, ArgAction};

#[derive(Parser)]
#[command(version = "1.0", about = "Remove a given substring from a string")]

struct Cli {
    // The input string
    input: String,

    // The substring to remove
    remove: String,

    // Case-insensitive flag
    #[arg(short, long, action = ArgAction::SetTrue)]
    case_insensitive: bool,
}


fn remove_string(input: &str, target: &str) -> String {
    input.replace(target, "")
}

fn main() {
    let args = Cli::parse();

    let result = if args.case_insensitive {
        remove_string(&args.input.to_lowercase(), &args.remove.to_lowercase())
    } else {
        remove_string(&args.input, &args.remove)
    };
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

    #[test]
    fn test_case_insensitive_removal() {
        let args = Cli::try_parse_from(["prog", "Hello World", "WORLD", "--case-insensitive"]).unwrap();
        let result = if args.case_insensitive {
            remove_string(&args.input.to_lowercase(), &args.remove.to_lowercase())
        } else {
            remove_string(&args.input, &args.remove)
        };
        assert_eq!(result, "hello "); // Case-insensitive should match "WORLD"
    }
}