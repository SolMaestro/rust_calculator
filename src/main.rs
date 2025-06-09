//THE MAIN.RS FILE SERVES AS THE ENTRY POINT TO THE CALCULATOR PROGRAM.

mod calculator;  //The mod keyword brings other modules to scope in the main file.
mod logic;

use calculator::calculate;                                                   //Since those modules are now in scope, the functions in those modules can be accessed with the use keyword and a path to the function.
use logic::{read_number, read_operator, read_continue, continue_or_quit};   //Just a another use keyword calling multiple functions to scope in a shorter form, instead of calling them one after the other.

fn main() {
    println!("Hello, Welcome to Rust Calculator!");         // This line prints a welcome message
    println!("Enter 'q' to quit or 'go' to continue");      // A prompt to quit or continue the program
     let quit_or_continue = continue_or_quit();       // This variable holds the user input and returns a bool value.

        if !quit_or_continue {                              //If the variable returns false
            println!("Calculator cancelled, Goodbye!");
            return;                                         // End the program by returning it early.                                
}
     loop {

    let num1 = read_number("Enter the first number: ");                  // Takes first value for the operation.
    let operator = read_operator("Choose operation (+, -, *, /, %): ");    // Asks for the operator.
    let num2 = read_number("Enter the second number: ");                 // Takes the second value for operation.

    match calculate(num1, num2, operator) {                                          //Since the calculate function is now in scope, the function can be called and the required arguments passed into it.
        Some(result) => println!("Result is: {}", result),                      //This line returns an option<f64> and creates a variable named `result`, that holds and prints the value.
        None => println!("Error: Invalid operation (cannot divide number by 0)"),   //If the match expression evaluates to a None, This message prints because of an invalid operation like dividing a number by 0.
    }

   if !read_continue("Do you want to perform another calculation?  (y/n)") {   //This user friendly evaluation function returns a bool and simply asks if the user wants to perform more calculations.
      println!("Goodbye!");                                                           //if yes the program loops again and calculations continue but if no, then goodbye message prints.
           break;                                                                     //break after evaluating to false.
      }
      println!("\n <--Starting a new calculation-->\n");
    }
      } //End of main code block.