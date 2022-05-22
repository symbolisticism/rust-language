// you have to import this crate for basic input/output
extern crate text_io;

/*
Function: main
Purpose:  the main driver of the program
*/
fn main() {

    // this chunk of code was taken from https://www.geeksforgeeks.org/standard-i-o-in-rust/

    // 'println!' is a macro in Rust, which means that it is a built-in function
    println!("Enter a name:");
    // 'let' is how you declare a normal variable
    //   the 'mut' keyword allows you to declare a variable that can
    //   be changed or redeclared later
    let mut guess = String::new(); // 'String::new() calls the constructor for a new string object
 
    // this is the line of code used for taking input
    //   it is rather complicated
    std::io::stdin().read_line(&mut guess).expect("failed to readline");
    
    // the 'print!' macro differs from the 'println!' macro only in that the
    //   the 'println!' macro appends a new line at the end of whatever it
    //   it prints to the console
    print!("You entered {}", guess); // inline string formatting is possible in Rust
    
    println!("\nEnter a number:");

    // declare a variable to hold the number input
    let mut x = String::new();
    // as you can see, the mutable variable 'x' is called by reference in the 
    //   input line code
    std::io::stdin().read_line(&mut x).expect("failed to readline");

    // I was not able to figure out how to take in input in integer format
    //   this was my attempt at casting the user input to an integer, but 
    //   that doesn't quite work in Rust
    // let sum = add(x as i32);

    // the '+ 2' was added here because there were some unpredictable things
    //   happening with the output, and I was just trying to get it to display right
    // print!("The sum of the numbers 1 to {} is {}\n", x + 2, sum);

    // just a simple display of logic/decision making in Rust
    // if sum < 1000 {
    //     println!("You should pick a bigger number.\n");
    // } else {
    //     println!("Are you surprised at how quickly numbers add up?\n");
    // }

    // when using the 'let' keyword to declare a variable, you have the option
    //   of specifying what data type the variable should be. this is an example
    //   of that with the colon and the keyword 'bool' after the declaration
    let town_is_big: bool = true;

    // the if statement is obviously unreachable code, but this is an example
    //   of the syntax of decision making in Rust using a boolean
    if !town_is_big {
        println!("Do you want to move away to a big city?\n");
    } else {
        println!("Soon enough your town will be a city!\n");
    }

    // make a simple array of data to be used for slicing example
    //   this can be a list or a string of text
    let sentence = "This is a good example of what a sentence is.".to_string(); // string

    let data = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]; // array

    show_slicing_sentence(&sentence);
    
    show_slicing_array(&data);

}

/*
Function: add
Purpose:  takes a number and adds up all the integers from one to
          to that number and returns the sum
*/
fn add(x: i32) -> i32 {

    let mut sum = 0;

    for i in 1..x + 3 {
        sum += i;
    }

    sum
}

/*
Function: show_slicing
Purpose:  demonstrates slicing in the Rust language with a
          a string
*/
fn show_slicing_sentence(slice: &str) {
    // display some facts about the sentence
    println!("\nThe length of the string is {}", slice.len());
    println!("This is the original string: {}", slice);
    // use the ampersand operator to reference the string
    //   and specify the indices you want to include in the 
    //   brackets. The end index is not included

    // make a slice of the sentence
    let piece = &slice[5..19];

    println!("The slice of the sentence from index 5 to 18 is: {}", piece);

}

/*
Function: show_slicing_array
Purpose:  demonstrates slicing in the Rust language with a
          a string
*/
fn show_slicing_array(arr: &[i32]) {
    // display some facts about the array
    println!("\nThe length of the array is {}", arr.len());
    println!("The original array is: {:?}", arr);
    // make a slice of the array
    let piece = &arr[2..7];

    println!("The slice of the array from index 2 to 6 is: {:?}", piece);
}