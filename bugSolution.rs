fn main() {
    let mut x = 5;
    { //Scope for the first mutable borrow
        let y = &mut x; 
        *y = 10; 
    }
    { //Scope for the second mutable borrow
       let z = &mut x;
       *z = 100; 
    }
    println!("x = {}", x);
}