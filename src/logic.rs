use std::io;                                    // Bringing the standard input and output module to scope helps the program handle user inputs and outputs efficiently.

pub fn read_number(prompt: &str) -> f64 {      // This function takes a string(&str) parameter that returns a floating point number. It is used to process and validate the operands.
    loop {
        println!("{}", prompt);                          // Prints the prompt for a user to enter a number
        let mut input = String::new();           // This line creates a new variable `input` that will store the user input value
        io::stdin().read_line(&mut input).unwrap();  // This is the actual line that takes user input and stores it in the input variable we created above

        match input.trim().parse::<f64>() {                     // Since user input is stored as Strings, this line parses them into an actual floating point number(f64). One can't do arithmetics with Strings right?
            Ok(val) => return val,                         // The parse method actually returns a Result<T, U>, hence the reason why we match with Ok and Err. If number entered is valid and not letters, this line returns the variable val that is used to grab the values after the parse.
            Err(_) => println!("Please enter a valid number."), // If invalid characters like letters or special characters are entered, the program returns the error code on this line. Only numbers are valid!
        }
    }
}                                                                   // End of read_number block scope

pub fn read_operator(prompt: &str) -> char {                     // This function is used to handle the operation characters. It takes a string(&str) parameter for prompting user and returns a character. it is used to check and verify the operator characters.
    loop {
        println!("{}", prompt);                                 // Prints the prompt to enter operator
        let mut input = String::new();                  // Creates a variable as usual to store the operator
        io::stdin().read_line(&mut input).unwrap();        // Takes the mutable user input
        
        if let Some(operator) = input.chars().next() {     // This is also an alternative to using the match expression. This line simply executes the code by checking if any value or values are found in the user input. it also trimms the input and converts it from Strings to chars before checking in the next line.
            if "+-*/".contains(operator) {                       // The .contains() method checks if user input contains a char, ofcourse it should be one of the any allowed operator.
                return operator;                                  // If true, the operator value gets returned.
            }
        }

        println!("Please enter a valid operator: +, -, *, /");     // If the expression above evaluates to true, Rust skips this code. If not, Rust prints this message that signifies an error i.e if the user entered letters and special characters as the operator character instead of the specified operator characters
    }
}                                                                  // End of read_operator block


//This block contains the logic for the functionality of allowing calculator user to exit the program early.
pub fn continue_or_quit() -> bool {
  loop {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
      "go" | "g" => return true,
      "quit" | "q"  => return false,
      _ => println!("Please enter 'quit' or 'go' to continue"),
  }
}
}

pub fn read_continue(prompt: &str) -> bool {                     // This last function in this module also takes a string parameter and returns a boolean
    loop {
        println!("{}", prompt);                                  // This line prints the prompt for the user to continue with the program by choosing yes or no
        let mut input = String::new();                   // Variable created to store input as usual
        std::io::stdin().read_line(&mut input).unwrap();    // Still accepting mutable user input and unwrapping it for the program to use

        match input.trim().to_lowercase().as_str() {            // This match expression transforms the user input with some methods and also starts running some checks
            "y" | "yes" => return true,                         // The program continues if this line gets executed
            "n" | "no" => return false,                         // If not the program should return a false which according to the program logic in main should exit the program
            _ => println!("Please enter 'y' or 'n'."),          // The line prints, if user enters an invalid choice.
        }
    }
}                                                               //End of read_continue block scope.