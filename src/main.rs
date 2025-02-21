use std::env;

fn remove_string(input: &str, target: &str) -> String {
    input.replace(target, "")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: str_cleaner <input_string> <substring_to_remove>");
        std::process::exit(1);
    }

    let input_string = &args[1];
    let substring_to_remove = &args[2];

    let result = remove_string(input_string, substring_to_remove);
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