/*
 * The following program demonstrates functions
 */
fn main() {
    println!("Hello, world!");
    //To call a function:
    another_function();
    // Function call with 1 arguement 
    function2(50);
    // Function call with 2 arguement
    function3(24, 'M');
    // x holds the returned value of the function
    let x = function4();
    // y holds the returned value of function which takes arguements
    let y = function5(5);
    println!("X is: {x}");
    println!("Y is: {y}");

}
/* The function can be declared before/ after main function. Its format is keyword, name (snake
 * case), parameters & function body
 */
fn another_function(){
    println!("2nd function");
}
/*
 *To declare a parameter which is a special variable, then it must always have var name & type
 */
fn function2(x: i32){
    println!("The value of x: {x}");
}
// To declare multiple parameters, separate with commas
fn function3(age: u8, gender:char){
    println!("I am {} years old & am {}", age, gender);
}
// To return a value specify return type after arrow.
fn function4()->i32{
    // Rust returns the last expression implicitly
    5
}
// A function with both parameters & return value7

fn function5(x:i32)->i32{
    // To return the value of a statement
    return x+1;
}

