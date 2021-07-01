fn main(){
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };  //代码块最后执行的表达式结果作为右值赋给左值

    let z = {
        2 * x;
    };  //代码块最后一条表达式结尾处有分号，那么右值变为()
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
 