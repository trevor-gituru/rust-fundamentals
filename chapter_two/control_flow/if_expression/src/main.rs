/*
 * Demonstrating use of if statement dor branching
 */
fn main() {
    let number = 8;
    // It is followed by condition which is a boolean that decideds which block executed
    if number>5{
        // True block
        println!(" {number} > 5");
    }else{
        //False block
        println!(" {number} < 5");
    }
    // We can have multiple conditions so that only 1 block of code is executed for the first one
    // to be true.
    if number % 4==0 {
        println!("{number} is divisible by 4");
    }else if number % 3==0 {
        println!("{number} is divisible by 3");
    }else if number % 2==0 {
        println!("{number} is divisible by 2");
    }else{
        println!("{number} is not divisible by 4,3  or 2");
    }
    // if can also be used as an expression to be binded to a variable. In this case, both the
    // returned values musf be of similar data types
    let number = if true {5} else { 8};
    println!("The value of number: {number}");
}
