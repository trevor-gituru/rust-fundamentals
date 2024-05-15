/*
 *  The following program demonstrates the concept
 *  of shadowing where one is able to declare a
 *  new variable with the same name as an
 *  existing one
 */
fn main() {
    // The 1st variable is declared
    let x = 5;
    println!("The value of 1st x: {x}");
    // The 2nd variable overshadows the 1st one
    let x = x+1;
    println!("The value of 2nd x: {x}");
    {
        //The 3rd variable overshadows the 2nd
        let x = x*2;
        println!("The value of 3rd x is: {x}");
    }
    // The scope of 3rd x ends hence x is 2nd one
    println!("The value of 2nd x is: {x}");
    /* 
     * The above demonstrates importance of 
     * shadowing in transforming the value of a
     * variable while having it immmutable
    */

    /* The 2nd advantage over mut is that it 
     * enables one to change the data type of variable while reusing the same name
     */
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");
    
}
