# Rust

## primitives 原生类型

1. scalar type 标量类型
    + `signed integers`：i8、i16、i32、i64 和 isize（指针宽度）
    + `unsigned integers`： u8、u16、u32、u64 和 usize（指针宽度）
    + `floating point`： f32、f64
    + `char`：单个 Unicode 字符，如 'a'，'α' 和 '∞'（都是4B）
    + `bool`：true or false
    + `unit type`：值为 () 空元组（单元素元组，不为复合类型）
2. compound type 复合类型
    + `array` 数组：如[1,2,3]
    + `tuple` 元组：如[1,true]
3. 定义变量

    ```rust
    let a_float: f64 = 1.0;     //常规说明类型
    let b_integer = 2i32;       //后缀说明类型
    let default_float = 3.0;    //默认f64、i32
    let mut inferred_type = 12;
    inferred_type = 123456i64;  //根据下一行赋值推断类型
    
    let mut mutable = 12;
    mutable = 666;      //mutable值可变
    //mutable = true; //错误！mutable的值可变，类型不可变
    let mutable = true; //可用shadow（掩蔽）的方法覆盖，更改类型
    ```

### literals and operators 字面量和运算符

```rust
//字面量和运算符
println!("无符号整数：1 + 2 = {}",1u32+2);
println!("亦或运算：0011 XOR 0101 is {:04b}",0b0011u32 ^ 0b0101);
println!("位运算：0x80 >> 2 is 0x{:x}",0x80u32 >> 2);
println!("一百万：One million is written as {}",1_000_000u32);
```

### tuple 元组

```rust
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;
    (boolean, integer)
}

//可debug打印的结构体
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "( {}, {} )\n( {}, {} )",
            self.0, self.1, self.2, self.3)
    }
}

fn main(){
    //tuples 元组
    let long_tuple = (1u8,2u16,3u32,4u64,
        -1i8,-2i16,-3i32,-4i64,
        0.1f32,0.2f64,
        'a',true);
    println!("long tuple first value: {}", long_tuple.10);   //下标访问元组
    let tuple_of_tuples = ((1u8,2u16,3u32),(4u64,-1i8),-2i16);
    println!("touple of touples:{:?}",tuple_of_tuples);     //{:?} fmt::Debug打印元组，很长元组无法打印

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct)，从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    
    //调用函数逆转元组
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));


    //创建结构体实例并打印
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Debug Matrix print：{:?}", matrix);
    println!("Display Matrix print：\n{}", matrix);
}
```

### array and slice 数组和切片

1. array 数组：在内存中连续存储的相同类型`T`的对象集合
    + [T;size]
2. slice 切片：类似数组，但大小在编时不确定，且为`双字对象`
    + &[T]
    + two-word object (word的宽度 = usize，取决于处理器架构)
        + first word 指向 数据的指针
        + second word 指向 切片的长度
    + slice可以用来借用数组的一部分
3. 运行测试

    ```rust
    //此函数借用一个slice
    fn analyze_slice(slice: &[i32]){
        println!("------------loop_slice------------");
        println!("slice size is {}", slice.len());
        for (count, i) in slice.iter().enumerate(){
            println!("slice[{}] is {}", count, i);            
        }
        println!("----------------------------------");
    }

    fn main(){
        println!("--------------array---------------");
        //array 数组
        let xs: [i32;5] = [1,2,3,4,5];  //类型i32，长度5
        let ys: [i32;500] = [6;500];    //初始值都赋为0
        println!("array[0] is {}", xs[0]);  //返回array元素
        println!("array size is {}", xs.len()); //返回array长度
        //数组在栈中分配，查看array占用大小
        println!("array occupies {} bytes", mem::size_of_val(&xs));
        //i32 即 4 bytes * 5 个元素 = 20 bytes 大小
        
        //slice 切片
        println!("--------------slice---------------");
        let slice = &xs[2 .. 4];
        println!("slice size is {}", slice.len());
        println!("slice[0] is {}", slice[0]);
    }
    /* 输出结果
    --------------array---------------
    array[0] is 1
    array size is 5
    array occupies 20 bytes
    --------------slice---------------
    slice size is 2
    slice[0] is 4
    ------------loop_slice------------
    slice size is 5
    slice[0] is 1
    slice[1] is 2
    slice[2] is 3
    slice[3] is 4
    slice[4] is 5
    ----------------------------------
    ------------loop_slice------------
    slice size is 3
    slice[0] is 6
    slice[1] is 6
    slice[2] is 6
    ----------------------------------
    */
    ```
