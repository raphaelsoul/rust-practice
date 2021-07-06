fn sum(a: i32, b: i32) -> i32 {
    a + b
}


fn main() {
    let mut fibonacci = Vec::from([0, 1]);
    for _ in 0..10 {
        let new_el = sum(fibonacci[fibonacci.len() - 1], fibonacci[fibonacci.len() - 2]);
        fibonacci.push(*&new_el);
        println!("{:?}", fibonacci)
    }

}
