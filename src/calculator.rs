pub fn calculate(num1: f64, num2: f64, operator: char) -> Option<f64> {    // This calculate function in this module takes parameters for operator calculations and returns an Option<f64> which evaluates to a Some or None.
    match operator {                                                       // Match the operator char values.
        '+' => Some(num1 + num2),                        // This line checks for the addition operator and returns a value as a Some.
        '-' => Some(num1 - num2),                        // This line checks for the difference operator and also returns a value as a Some.
        '*' => Some(num1 * num2),                        // This line also checks for the product and returns a value as a Some. 
        '/' => {                                         // Lastly this line checks for the quotient operator.
            if num2 == 0.0 {                            // If the last quotient value is equal to 0 or 0.0,
                None                                    // The check returns a None.
            } else { 
                Some(num1 / num2)                     //If the check sees a value above 0 or 0.0, it goes ahead with the division operation and returns a Some.
            }
        }
        _ => None,                                  //This line returns a None if all the Some checks fails.
    } 
        }                                         //End of calculate function code block.