/* 
 * The following program demonstrates the data type integer which is a number without fractional
 * component
 */
fn main() {
    //The for loop prints the range of the 8, 16, 64 bits in both signed and unsigned
    for i in 0..4 {
        let n: u32 = 2u32.pow(i+3);
        let imax: i128 = 2i128.pow(n-1)-1;
        let imin: i128 = -2i128.pow(n-1);
        let umax: u128 = 2u128.pow(n)-1;
        let umin: u128 = 0;
        println!("For u{n} its range is {umin} - {umax}");
        println!("For i{n} its range is {imin} - {imax} \n");
    }
    // The following prints the range of 128-bit value in both signed & unsigned
    let u128_max = u128::MAX;
    let u128_min = u128::MIN;
    let i128_max = i128::MAX;
    let i128_min = i128::MIN;
    println!("For u128 its range is {u128_min} - {u128_max} ");
    println!("For i128 its range is {i128_min} - {i128_max} \n");
    // The following prints the length of the current architecture of the computer
    let n = std::mem::size_of::<usize>() * 8;
    println!("For usize & isize, it depends on the architecture of ones computer. Then, the length in bits of my computer is {n}-bits\n");
    //Number literals
    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byt = b'A';
    println!("Number literals are as follows: \nDecimal: {}\nHexadecimal: {}\nOctal: {}\nBinary: {}\nByte: {} \n", dec, hex, oct, bin, byt);
    
}
