// This is a comment

// MACROS look like functions, except their name ends with a bang '!'
// instead of generating a function call, macros are expanded into
// source code that gets compiled with the rest of the program.

// Macros are created using the 'macro_rules!' macro.
macro_rules! say_hello {
    //  '()' indicates that the macro takes no arguments.
    () => {
        // The macro will expand into the contents of the exercices.
        println!("Hello!")
    };

    // Macros are useful: 
    // 1. Don't repeat yourself
    // 2. Domain-specific languages.
    // 3. Variadic interfaces.
}

// Main function.
fn main(){

    // Statements - executed when the compiled binary is called
    say_hello!();

    println!("Hello World!")
    // pringln! is a ~macro~ that prints text to the console
    // In this case, it takes an argument.

}
