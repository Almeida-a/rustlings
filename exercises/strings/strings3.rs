// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// I AM DONE

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

fn compose_me(input: &str) -> String {
    // Add " world!" to the string! There's multiple ways to do this!
    String::from(input.to_owned() + " world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // Plan: define old and new strings
    //      define a counter: when reaches old.length => match! && counter = 0
    let mut char_vec: Vec<char> = Vec::new();
    let mut tmp_vec: Vec<char> = Vec::new();

    let mut i: usize = 0;

    let new = "balloons";
    let old = "cars";

    let mut old_vec: Vec<char> = old.chars().collect::<Vec<char>>();

    for c in input.chars() {
        if c == old_vec[i] {
            i += 1;
            if i == old.chars().count() {
                // Match! Push new to char_vec
                for nc in new.chars() {
                    char_vec.push(nc);
                }
                i = 0;
                tmp_vec.clear();
                continue;
            }
            tmp_vec.push(c);
        } else {
            if i != old.chars().count() {
                char_vec.extend_from_slice(tmp_vec.as_slice());
                tmp_vec.clear();
            }
            char_vec.push(c);
            i = 0;
        }
    }

    char_vec.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
