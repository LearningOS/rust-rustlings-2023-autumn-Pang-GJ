// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use std::future::IntoFuture;

    use super::Command;

    // // my version
    // pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
    //     // TODO: Complete the output declaration!
    //     let mut output: Vec<String> = vec![];
    //     for (string, command) in input.iter() {
    //         let output_string = match command {
    //             Command::Uppercase => string.to_uppercase().clone(),
    //             Command::Trim => string.trim().to_string().clone(),
    //             Command::Append(size) => {
    //                 let mut num = *size as i32;
    //                 let mut tmp = string.clone();
    //                 while num != 0 {
    //                     tmp.push_str("bar");
    //                     num -= 1;
    //                 }
    //                 tmp
    //             }
    //         };
    //         output.push(output_string);
    //     }
    //     output
    // }

    // chatgpt version
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = Vec::with_capacity(input.len()); // 预分配
        for (string, command) in input.iter() {
            let output_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(size) => {
                    let bar = "bar".repeat(*size);
                    format!("{}{}", string, bar)
                }
            };
            output.push(output_string);
        }
        output
    }

    // chatgpt version, stream style
    // pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
    //     input
    //         .into_iter()
    //         .map(|(string, command)| match command {
    //             Command::Uppercase => string.to_uppercase(),
    //             Command::Trim => string.trim().to_string(),
    //             Command::Append(size) => string + &"bar".repeat(size),
    //         })
    //         .collect()
    // }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::Command;
    use crate::my_module::transformer;

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
