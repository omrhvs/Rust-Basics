// Single line comment.

/* 
    Multiple lines comment.
    hello.
*/

use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main(){
    println!("Hola Mundo!");

    // Variables - 'let' keyword for declaring variables.
    let my_string: &str = "Text";

    // We cannot print strings directly.
    println!("Variable: {}", my_string);
    println!("Variable: {my_string}");

    /*
        &str is an inmmutable variable, that means
        that we cannot do the following:

        my_string = "Now this is the text";

        Unless we do the next.
     */ 

    let mut my_string = "Text B";
    println!("{}", my_string);
    my_string = "Now Text B";

    println!("Mutable Variable: {}", my_string);

    /* 
        If we declare and re-assign the value, the IDE will
        warn me, unless we declare, use, and then re-assign.
    */

    let _my_string2: String;

    /*
        If I happen to declare an unused variable, it will mark a 
        warn, to get rid of it, I have to use an '_' at the beggining
        to indicate that it is intentional.

        &str - uses the maximum memory size a string can have.
        String - maybe the most optimal, we can adjust its memory size.

        With the second one, we need to initialize.
    */ 
    let my_string3: String = String::from("Text C");

    println!("{my_string3}");

    /*
        About integers, we can declare signed integers or unsigned (i8, i16,
        i32...isize or u8, u16, u32...usize)
    */
    let mut my_int: i32 = 7;
    my_int = my_int + 4;

    println!("{}", my_int);

    // Operations with integers
    println!("{}", my_int - 1);


    let my_int64: i64 = 7;
    println!("{my_int64}");

    /*
        Creating an integer is by default 32 bits alike a decimal number
        which by default is 64 bits.
    */

    // Decimals
    let my_float: f64 = 6.5;
    println!("{my_float}");

    /*
        You can't mix decimals and integers directly. This will throw an
        error:

        my_float = my_float + my_int;
        println!("{my_float}");

        We would need to parse values.
    */ 

    // Booleans
    let mut my_bool: bool = false;
    println!("{my_bool}");
    my_bool = true;
    println!("{my_bool}");

    // Constants
    const MY_CONST: &str = "My constant property";

    /*
        As we know, in most programming languages, it's a common practice
        to declare constants with CAPS syntax.
    */

    println!("{MY_CONST}");

    // Control Flow
    if my_int == 10 {
        println!("Value is 10.")
    } else {
        println!("Value is not 10.")
    }

    // Lists
    let mut my_list: Vec<&str>  = vec!["Omar", "Hernandez", "@omrhvs", "21"];
    
    /* 
        To print a list, we use ':?' before the variable name, for
        instance:
     */
    println!("{:?}", my_list);

    /*
        We can't mix different data types, this would cause an error:
        let my_list2: Vec<&str>  = vec!["Omar", "Hernandez", "@omrhvs", 21];

        To add an element to a list we use '.push'. Again, we can only do this
        if the variable is mutable.
    */

    my_list.push("Venegas");
    println!("{:?}", my_list);

    // To print specific elements.
    println!("First element: {:?}", my_list[0]);
    
    // Sets
    let my_set: HashSet<&str> = vec!["Omar", "Hernandez", "@omrhvs"].into_iter().collect();
    println!("A: {:?}", my_set);

    /*
        Sets don't have order, so if we insert a new element, it will display
        on a different position every execution. It also don't allow repeated
        elements.
    */
    let mut my_set2: HashSet<&str> = vec!{"Omar", "Hernandez", "@omrhvs"}.into_iter().collect();
    my_set2.insert("Test");
    println!("B: {:?}", my_set2);

    // Maps
    let my_map: HashMap<&str, i32> = vec![("Omar", 21), ("Hernandez", 22), ("Venegas", 23)].into_iter().collect(); // Unordered key values
    println!("{:?}", my_map);

    let mut my_map2: HashMap<&str, i32> = vec![("Omar", 21), ("Hernandez", 22), ("Venegas", 23)].into_iter().collect(); // Unordered key values
    my_map2.insert("Blas", 26);
    println!("{:?}", my_map2);

    // Loops
    let my_list3: Vec<&str>  = vec!["Omar", "Hernandez", "@omrhvs", "21"];
    println!("List looped:");
    for value in &my_list3 { // We use pointers referencing the list.
        println!("{}", value);
    }

    let my_set3: HashSet<&str> = vec!["Omar", "Hernandez", "@omrhvs"].into_iter().collect();
    println!("Set looped:");
    for value in &my_set3 {
        println!("{}", value);
    }

    let my_map2: HashMap<&str, i32> = vec![("Omar", 21), ("Hernandez", 22), ("Venegas", 23)].into_iter().collect(); // Unordered key values
    println!("Map looped:");
    for (key, value) in &my_map2 {
        println!("{} {}", key, value);
    }

    // While
    let mut my_counter = 0;

    println!("Using while:");
    while my_counter < my_list3.len() {
        println!("{}", my_list3[my_counter]);
        my_counter +=1;
    }

    // Function
    my_function();

    // Structure
    let my_struct = MyStruct::new("Omar", 21);  // We build our structure
    println!("Structure: {}, {}", my_struct.name, my_struct.age);

}

// Functions
fn my_function(){
    println!("This is a function.");
}

// Structures
struct MyStruct {
    name: String,
    age: i32,
}

// Implementations
impl MyStruct {
    fn new(name: &str, age: i32,) -> MyStruct {
        MyStruct {
            name: String::from(name),
            age
        } // This is like a constructor
    }
}
