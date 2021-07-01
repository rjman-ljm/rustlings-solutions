//不显示类型转换产生的溢出警告
#![allow(overflowing_literals)]
//允许非驼峰式起别名
#![allow(non_camel_case_types)]

//为u64起别名
type Inch = u64;
type u64_t = u64;   //宏允许非驼峰式别名

fn main(){
    //casting 类型转换
    let decimal = 65.4321f32;
    //as关键字实现显式类型转换
    let integer = decimal as u8;
    let _character = integer as char;
    //不提供如下隐式类型转换
    //let integer: u8 = decimal;
    
    //unsigned number and signed number
    let num = 1000;
    println!("{}：{:b} as u32 is {:032b}", 
        num, num, num as u32);
    println!("{}：{:b} as u8 is {:8b}", 
        num, num, num as u8);   //类似对2^8取模
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);  

    //inference 推断
    //elem类型为u8
    let elem = 5u8;
    //只知道vec某东西构成的向量Vec<_>，不知道具体类型
    let mut vec = Vec::new();
    //编译器推断得到vec为u8类型的向量
    vec.push(elem);
    println!("vec：{:?}", vec);

    let inch: Inch = 2 as u64_t;
    println!("inch = {}", inch);
}