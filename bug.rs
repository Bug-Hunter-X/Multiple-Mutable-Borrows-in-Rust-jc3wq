fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; // This line works fine
    let z = y; 
    *z = 100; //This line will cause an error
    println!("x = {}", x);
}