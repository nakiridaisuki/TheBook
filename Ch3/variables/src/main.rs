fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");

    const TREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("Tree hours in second is {TREE_HOURS_IN_SECOND}");

    let x = 5;
    let x = x + 1;
    {
        let x = x + 1;
        println!("local x is {x}");
    }
    println!("x is {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces length is {spaces}");
}
