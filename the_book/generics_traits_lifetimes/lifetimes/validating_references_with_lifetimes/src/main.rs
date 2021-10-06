// * Validating References with Lifetimes


// ! Preventing Dangling References with Lifetimes
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
