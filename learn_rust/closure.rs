fn main() {
    let add_one_v1 = |x: u32| -> u32 { x+1 };
    let add_one_v2 = |x: u32| { x+1 };
    let add_one_v3 = |x: u32| x+1;
    println!("v1 = {}", add_one_v1(1));
    println!("v2 = {}", add_one_v2(1));
    println!("v3 = {}", add_one_v3(1));
}
