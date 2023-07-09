// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// Execute `rustlings hint quiz2` or use the `hint` watch subcommand for a hint.

// I AM DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            if let Command::Uppercase = command {

                output.push(string.to_uppercase());

            } else if let Command::Trim = command {

                output.push(trim_me(string));

            } else if let Command::Append(times) = command {
                let appendee = "bar";
                let mut string = string.to_owned();
                for _ in 0..*times {
                    string = string + appendee;
                }
                output.push(string.clone());
            }
        }
        output
    }

    fn trim_me(input: &str) -> String {
        // Remove whitespace from the end of a string!
        let mut char_vec: Vec<char> = Vec::new();
    
        let mut is_to_trim = true;
        // Trim the white spaces to the left
        for c in input.chars() {
            if is_to_trim {
                if c != ' ' {
                    is_to_trim = false;
                    char_vec.push(c);
                }
            } else {
                char_vec.push(c);
            }
        }
    
        // Trim the white spaces to the right
        loop {
            let vec_char = char_vec.pop().unwrap();
            if vec_char != ' ' {
                char_vec.push(vec_char);
                break;
            }
        }
    
        char_vec.iter().collect::<String>()
    }
    
}

#[cfg(test)]
mod tests {
    // TODO: What to we have to import to have `transformer` in scope?
    use super::my_module::*;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
