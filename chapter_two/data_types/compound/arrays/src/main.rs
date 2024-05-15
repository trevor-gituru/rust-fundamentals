/*
 * The following program demonstrates the arrayndata type which is a collect of values of similar
 * data type
 */
fn main() {
    //To declare a variable array bases on the value
    let a = [1,2,3,4,5];
    // To declare with type annotation & length
    let b: [i32;5] = [6,7,8,9,10];
    // To initialize an array to have n elements each same value    
    let c = [3; 5];
    //Accessing array elements
    let first = a[0];
    let second = a[1];
    // Print values
    println!("The a array is: {:?}\nThe b array is: {:?}\nThe c array is: {:?}\n",a,b,c);
    println!("The first & second element of a: {} & {} ", first, second);
}
