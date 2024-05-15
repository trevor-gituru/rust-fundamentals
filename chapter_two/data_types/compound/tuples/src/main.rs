/*
 * This program demonstrates tuple datat types which is a compound of different values of different
 * data typea
 */
fn main() {
    // To declare immutable tuple with explicit type annotation
    let tup1: (i32, f64, u8) = (500, 6.8, 1);
    //To declare mutable tuple based on value
    let mut tup2 = (500, 6.8, 1);
    // To declare an empty tuple known as unit
    let tup3 =();
    //Destructuring a tuple with pattern matching
    let (x, y, z) = tup1;
    // Indexing a tuple 
    let m1 = tup2.0;
    let m2 = tup2.1;
    let m3 = tup2.2;
    // Changing value of mutable tuple
    tup2.2 = 5;
    // Displaying values
    println!("The first tuple: {:?}\nThe second tuple: {:?}\nThe unit tuple: {:?}\n", tup1, tup2, tup3);
    println!("Result of destructuring tup1:\nx: {:?}\ny: {:?}\nz: {:?}",x,y,z);
    println!("Result of indexing tup2:\nm1: {:?}\nm2: {:?}\nm3: {:?}",m1,m2,m3);


}
