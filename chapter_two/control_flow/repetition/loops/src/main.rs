/*
 * Demonstrating the loop repetition
 */
fn main() {
    // A counter is usually used to be able to step loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        // The condition to stop the loop & implicitly return a value
        if counter == 10 {
            break counter*2;
        }
    };
    println!("Result: {result}");
   /*
    * Labels can be used to name loops so as to break out of outer loop instead of an inner one. This
    * is shown in the code to print 2 numbers for char A-C
    */
    let mut chara = 'A';
    // Outer loop label as shown below
    'outer: loop{
        let mut inner = 2;
        print!("\n{chara}: ");
        loop{
            if inner == 0{
                break;
            }
            if chara == 'D'{
                break 'outer;
            }
            print!("{inner}  "); 
            inner -= 1;
        }
        // To go to next character
        chara = ((chara as u8)+1) as char;
    }
}
