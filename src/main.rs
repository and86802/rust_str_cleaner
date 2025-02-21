use clap::{Parser, ArgAction, Subcommand};

#[derive(Parser)]
#[command(version = "1.0", about = "A simple string manipulation tool")]

struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    // Remove a substring from the input string
    Remove {
        // The input string
        input: String,

        // The substring to remove
        remove: String,

        // Case-insensitive flag
        #[arg(short, long, action = ArgAction::SetTrue)]
        case_insensitive: bool,
    },

    // Replace a substring with another string
    Replace {
        // The input string
        input: String,

        // The substring to replace
        replace: String,

        // The replacement string
        with: String,

        // Case-insensitive flag
        #[arg(short, long, action = ArgAction::SetTrue)]
        case_insensitive: bool,
    }
}

fn remove_string(input: &str, target: &str) -> String {
    input.replace(target, "")
}

fn replace_string(input: &str, target: &str, replacement: &str) -> String {
    input.replace(target, replacement)
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Remove {input, remove, case_insensitive} => {
            let result = if case_insensitive {
                remove_string(&input.to_lowercase(), &remove.to_lowercase())
            } else {
                remove_string(&input, &remove)
            };
            println!("{}", result);
        }
        Commands::Replace {input, replace, with, case_insensitive} => {
            let result = if case_insensitive {
                replace_string(&input.to_lowercase(), &replace.to_lowercase(), &with)
            } else {
                replace_string(&input, &replace, &with)
            };
            println!("{}", result);
        }
    }
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
        let args = Cli::try_parse_from(["prog", "remove", "Hello World", "WORLD", "--case-insensitive"]).unwrap();
        if let Commands::Remove { input, remove, case_insensitive } = args.command {
            let result = if case_insensitive {
                remove_string(&input.to_lowercase(), &remove.to_lowercase())
            } else {
                remove_string(&input, &remove)
            };
            assert_eq!(result, "hello "); // Case-insensitive should match "WORLD"
        } else {
            panic!("Expected Remove command");
        }
    }

    #[test]
    fn test_case_insensitive_replacemant() {
        let args = Cli::try_parse_from(["prog", "replace", "Hello World", "WORLD", "disneyland", "--case-insensitive"]).unwrap();
        if let Commands::Replace { input, replace, with, case_insensitive } = args.command {
            let result = if case_insensitive {
                replace_string(&input.to_lowercase(), &replace.to_lowercase(), &with)
            } else {
                replace_string(&input, &replace, &with)
            };
            assert_eq!(result, "hello disneyland"); 
        } else {
            panic!("Expected Replace command");
        }
    }
}