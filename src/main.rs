use trsut::{add, mul, div};

fn main() {
    println!("Hello, world!");
    
    let sum = add(10, 5);
    println!("10 + 5 = {}", sum);
    
    let product = mul(10, 5);
    println!("10 * 5 = {}", product);
    
    let quotient = div(10, 5);
    println!("10 / 5 = {}", quotient);
}
