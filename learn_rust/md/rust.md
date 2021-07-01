# rust学习

## 环境配置

1. [rustlings](https://github.com/rust-lang/rustlings)
2. [学习rust](https://www.rust-lang.org/zh-CN/learn)
3. [win下Linux子系统wsl](https://www.jianshu.com/p/6b02948b3d37)
4. windows 和 linux子系统的剪切板通信问题
    + windows下的wsl根目录，找到相应路径的文件进行更改即可
    + `C:\Users\%USERNAME%\AppData\Local\Packages\CanonicalGroupLimited.UbuntuonWindows_79rhkp1fndgsc\LocalState\rootfs`
    + 建立`rjman`文件夹，linux下`cd`进入交换数据，bash中`右键`粘贴
    + !!!直接右键好像就解决了=_=

## 学习Rust的途径

1. [官方文档](https://doc.rust-lang.org/stable/rust-by-example/hello.html)
2. [中文翻译](https://rustwiki.org/zh-CN/rust-by-example/hello/comment.html)
3. [Rust官网](https://www.rust-lang.org/zh-CN/learn)
4. [菜鸟教程](https://www.runoob.com/rust/cargo-tutorial.html)
5. rustling

### rust和rustling安装

1. 安装[rust](https://rustup.rs/)
    + [windows端](https://win.rustup.rs/x86_64)
    + wsl端 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    + linux/MacOS端
2. 安装[rustlings](https://github.com/rust-lang/rustlings)
    + 依赖：git、rust（cargo）

### cargo使用

Cargo 是 Rust 的构建系统和包管理器

1. 构建 `cargo build`
2. 运行 `cargo run`

#### 在 VSCode 中配置 Rust 工程

```rust
cargo new greeting
cd greeting
mkdir .vscode
cd .vscode
touch tasks.json
touch launch.json
vim tasks.json
vim tasks.json
```

1. 项目下新建文件夹 `.vscode` 存放配置文件
2. 打开并新建配置文件 `tasks.json` 和 `launch.json`
    + taks.json

    ```json
    { 
        "version": "2.0.0", 
        "tasks": [ 
            {
                "label": "build", 
                "type": "shell", 
                "command":"cargo", 
                "args": ["build"]
            }
        ]
    }
    ```

    + launch.json

    ```json
    { 
            "version": "0.2.0", 
            "configurations": [ 
                { 
                    "name": "(Windows) 启动", 
                    "preLaunchTask": "build", 
                    "type": "cppvsdbg", 
                    "request": "launch", 
                    "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe", 
                    "args": [], 
                    "stopAtEntry": false, 
                    "cwd": "${workspaceFolder}", 
                    "environment": [], 
                    "externalConsole": false 
                }, 
                { 
                    "name": "(gdb) 启动", 
                    "type": "cppdbg", 
                    "request": "launch", 
                    "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe", 
                    "args": [], 
                    "stopAtEntry": false, 
                    "cwd": "${workspaceFolder}", 
                    "environment": [], 
                    "externalConsole": false, 
                    "MIMode": "gdb", 
                    "miDebuggerPath": "C:/cygwin64/bin", 
                    "setupCommands": [ 
                        { 
                            "description": "为 gdb 启用整齐打印", 
                            "text": "-enable-pretty-printing", 
                            "ignoreFailures": true 
                        } 
                    ] 
                } 
            ] 
        }
    ```

## format 格式化

1. 打印操作由 std::fmt 里面所定义的一系列`宏`来处理
    + `宏`并不产生**函数调用**，而是**展开成源码**，并和程序的其余部分一起被编译。Rust 又有一点和 C 以及其他语言都不同，那就是 Rust 的宏会展开为抽象语法树（AST，abstract syntax tree），而不是像字符串预处理那样直接替换成代码，这样就不会产生无法预料的优先权错误

    + 宏是通过 macro_rules! 宏来创建的

        ```rust
        // 这是一个简单的宏，名为 `say_hello`。
        macro_rules! say_hello {
            // `()` 表示此宏不接受任何参数。
            () => (
                // 此宏将会展开成这个代码块里面的内容。
                println!("Hello!");
            )
        }

        fn main() {
            // 这个调用将会展开成 `println("Hello");`!
            say_hello!()
        }
        ```

    + 宏的作用
        + **不写重复代码**（DRY，Don't repeat yourself.）
            + 很多时候你需要在一些地方针对不同的类型实现类似的功能，这时常常可以使用宏来避免重复代码
        + **领域专用语言**（DSL，domain-specific language）
            + 宏允许你为特定的目的创造特定的语法
        + **可变接口**（variadic interface）
            + 有时你需要能够接受不定数目参数的接口，比如 println!，根据格式化字符串的不同，它需要接受任意多的参数
2. `std::fmt`包含多种 `traits`（特征，特性） 来控制文字显示，其中重要的两种 trait 的基本形式如下
    + `fmt::Debug`：使用 `{:?}` 标记。格式化文本以供**调试**使用
    + `fmt::Display`：使用 `{}` 标记。以更优雅和**友好**的风格来格式化文本
3. 标准库提供了基本类型的`fmt::Display`的实现，如果需要打印自定义结构体，需要更多步骤
    + 标准库提供
        + `format!`：将格式化文本写到字符串（`String`）。（注：字符串是返回值不是参数）
        + `print!`：与 format! 类似，但将文本输出到控制台（`io::stdout`）
        + `eprint!`：与 format! 类似，但将文本输出到标准错误（`io::stderr`）
    + 自定义结构体
        + 如 `println!("This struct：{}", Structure(3));`是错误的
4. 格式化输出
    + `{0:>04b}`和`{0:>.05b}`
        + `0`：第1个参数
        + `:`：指示后面跟特殊格式
        + `0`：用零补全
        + `4`：整数占据4位
        + `.5`:小数占据5位
        + `<`、`^`、`>`：指示左、中、右对齐
        + `x`、`o`、`b`：指示16、8、2进制
    + `{0}`：指示第0个参数
    + `{name}`：命名参数
    + 精度
        + 整数（忽略精度）
        + 浮点数
            + `.5`：小数精度为5位
            + `.1$`：需要格式化**输入**参数中有一个`usize`作为精度
                + `1`：表示第2个参数(打印内容)
                + `$`：通配符(usize)
            + `.*`：需要格式化**输入**参数中有`usize`和`打印内容`
                + 打印内容
                + usize

            ```rust
            let pi = 3.1415926;
            println!(".N 表示精度：Pi is roughtly {0:>.3}",pi);         //.N表示精度
            println!(".N$ 表示精度：Pi is {:.1$}", pi, 5);      //.N$表示精度
            println!(".* 表示精度：Pi is {:.*}", 6, pi);        //.*表示精度
            println!("Pi is roughtly {:<04}， 0.5 is {:^04} of {:>08b}", pi, 1, 2);   //:04b 占4位其余补零，b指示为二进制
            ```

### fmt::Debug

1. 所有类型都能推导（`derive`，即自动创建）`fmt::Debug`的实现,但是，`fmt::Display` 需要手动实现

    ```rust
    // 推导 `Structure` 的 `fmt::Debug` 实现
    //否则显示 Structure` cannot be formatted using `{:?}`
    #[derive(Debug)]
    struct Structure(i32);  //包含单个 `i32` 的结构体。

    #[derive(Debug)]    //使 `Deep` 也能够打印
    struct Deep(Structure); //将 `Structure` 放到结构体 `Deep` 中

    fn main(){
        // `Structure` 也可以打印！(#[derive(Debug)])
        println!("Now {:?} will print!", Structure(3));
        // 使用 `derive` 的一个问题是不能控制输出的形式，如只想展示一个`7`该怎么实现？=> fmt::Display
        println!("Now {:?} will print!", Deep(Structure(7)));
    }
    ```

2. debug的美化打印（待理解`'a`）

    ```rust
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    fn main() {
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };

        println!("{:#?}", peter);   // 美化打印
    }
    //输出
    //Person {
    //    name: "Peter",
    //    age: 27,
    //}
    ```

### fmt::Display

1. 为自定义结构手动实现`fmt::Display`
    + 单个元素

        ```rust
        // （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
        use std::fmt;

        // 定义一个包含单个 `i32` 元素的结构体
        struct Structure(i32);

        // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` 的 trait
        impl fmt::Display for Structure {   //implement 实现
            // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
                // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
                write!(f, "{}", self.0)
            }
        }
        ```

    + 多个元素
        + 带有两个数字的结构体

            ```rust
            //推导出 `Debug`，以便与 `Display` 的输出进行比较
            #[derive(Debug)]
            struct MinMax(i64, i64);

            // 实现 `MinMax` 的 `Display`。
            impl fmt::Display for MinMax {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    // 使用 `self.number` 来表示各个数据。
                    write!(f, "({}, {})", self.0, self.1)
                }
            }    
            ```

        + 定义一个含有具名字段的结构体

            ```rust
            #[derive(Debug)]
            struct Point2D {
                x: f64,
                y: f64,
            }

            // 类似地对 `Point2D` 实现 `Display`
            impl fmt::Display for Point2D {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    // 自定义格式，使得仅显示 `x` 和 `y` 的值。
                    write!(f, "x: {}, y: {}", self.x, self.y)
                }
            }
            ```

        + 运行测试

            ```rust
            fn main() {
                //Debug
                println!("Debug print {:?}", Structure(3));     //Debug打印自定义结构体
                // 使用 `derive` 的一个问题是不能控制输出的形式，如只想展示一个`7`该怎么实现？
                println!("Debug print {:?}", Deep(Structure(7)));            

                println!("比较Debug和Display：");
                let big_range = MinMax(99,999);
                let small_range = MinMax(1,9);
                let point = Point2D{x: 3.3, y: 6.6};
                //Debug
                println!("Debug：print {:?} and {:?}",big_range, small_range);
                println!("Debug：print {:?}", point);
                //Display
                println!("print {big} and {small}", big = big_range, small = small_range);
                println!("print {}", point);
                
                // 报错，`Debug` 和 `Display` 都被实现了，但 `{:b}` 还需要 `fmt::Binary`得到实现
                // println!("What does Point2D look like in binary: {:b}?", point);
            }
            ```

2. 实现复数Complex结构的的fmt::Debug和fmt::Display

    ```rust
    #[derive(Debug)]    //为Complex实现fmt::Debug
    struct Complex{ //定义复数结构（re+im*i)
        re: f64,
        im: f64,
    }

    impl fmt::Display for Complex{  //为Complex实现Display
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "{} + {}i", self.re, self.im)
        }
    }

    fn main(){
        let complex = Complex{re: 1.2, im: 3.4};
        println!("Debug print Complex：{:?}",complex);
        println!("Display print Complex：{}",complex);
    }
    ```

3. `trait`是对未知类型 Self 定义的方法集，该类型也可以访问同一个 trait 中定义的其他方法

### List的fmt::Debug和fmt::Displays实现

1. List结构

    ```rust
    #[derive(Debug)]        //为List实现fmt::Debug
    struct List(Vec<i32>);  //定义包含单个 `Vec` 的结构体 `List`
    ```

2. 实现List结构的的fmt::Debug和fmt::Display

    ```rust
    impl fmt::Display for List{ //为List实现fmt::Display
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            let vec = &self.0;
            write!(f,"[")?; //向输出流f中写入`[`
            for(count, v) in vec.iter().enumerate(){
                if count != 0 { write!(f, ",")?;}   //除第一个元素外都加上逗号`，`
                write!(f, "{}: {}", count, v)?;    //向输出流f中写入值
            }
            // 加上配对中括号，并返回一个 fmt::Result 值。
            write!(f, "]")
        }
    }
    ```

### City结构体的fmt::Debug和fmt::Displays实现

1. City结构

    ```rust
    #[derive(Debug)]
    struct City{
        name: &'static str, //城市名称
        lat: f32,   //纬度
        lon: f32,   //经度
    }
    ```

2. 实现City结构的的fmt::Debug和fmt::Display

    ```rust
    impl fmt::Display for City{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            //write!和format类似，将格式化字符串写入缓冲区 f
            write!(f, "{}: {}°{}, {}°{}", self.name,
                self.lat, lat_c, self.lon, lon_c)
        }
    }
    ```

3. 测试输出

    ```rust
    fn main(){    
        let city = City{ name: "HangZhou", lat: 30.0, lon: 120.0 };
        println!("Debug City print：{:?}", city);
        println!("Display City print：{}", city);

        println!("----------loop_city----------");
        for city in [
            City{ name: "Dublin", lat: 53.347778, lon: -6.259772 },
            City{ name: "Oslo", lat: 59.95, lon: 10.75 },
            City{ name: "Vancouver", lat: 49.25, lon: -123.1 },
        ].iter() {
            println!("Debug City print：{:?}", *city);
            println!("Display City print：{}", *city);
        }
    }
    //输出结果
    //----------loop_city----------
    //Debug City print：City { name: "Dublin", lat: 53.34778, lon: -6.259772 }
    //Display City print：Dublin: 53.34778°N, -6.259772°W
    //Debug City print：City { name: "Oslo", lat: 59.95, lon: 10.75 }
    //Display City print：Oslo: 59.95°N, 10.75°E
    //Debug City print：City { name: "Vancouver", lat: 49.25, lon: -123.1 }
    //Display City print：Vancouver: 49.25°N, -123.1°W
    ```

### Color结构体的fmt::Debug和fmt::Displays实现

1. Color结构

    ```rust
    #[derive(Debug)]
    struct Color{
        red: u8,
        green: u8,
        blue: u8,
    }
    ```

2. 实现Color结构的的fmt::Debug和fmt::Display

    ```rust
    impl fmt::Display for Color{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "RGB({},{},{}) 0x{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue,
                self.red, self.green, self.blue)
        }
    }
    ```

3. 测试输出

    ```rust
    fn main(){
        let color = Color{ red: 223, green: 52, blue: 77 };
        println!("Debug Color print:{:?}", color);
        println!("Display Color print:{}", color);    
        println!("----------loop_color----------");
        for color in [
            Color { red: 128, green: 255, blue: 90 },
            Color { red: 0, green: 3, blue: 254 },
            Color { red: 0, green: 0, blue: 0 },
        ].iter() {
            println!("Debug Color print：{:?}", *color);
            println!("Display Color print：{}", *color);
        }
    }
    //输出结果----------loop_color----------
    //Debug Color print：Color { red: 128, green: 255, blue: 90 }
    //Display Color print：RGB(128,255,90) 0x80FF5A
    //Debug Color print：Color { red: 0, green: 3, blue: 254 }
    //Display Color print：RGB(0,3,254) 0x0003FE
    //Debug Color print：Color { red: 0, green: 0, blue: 0 }
    //Display Color print：RGB(0,0,0) 0x000000
    ```

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

## custom types 自定义类型

1. 关键字`struct`：定义一个结构体（`structure`）
2. 关键字`enum`：定义一个枚举类型（`enumeration`）
3. 常量`constants`可以通过关键字`const`和`static`创建

### structures

1. 单元结构体
    + `struct Nil;`（不带字段，在泛型中很有用）
2. 元组结构体
    + `struct Pair(i32,f32);`（具名元组）
3. C语言风格结构体
    + 结构体可作为另一结构体的字段

    ```rust
    #[derive(Debug)]
    struct Point2D{   //C风格结构体
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Rectangle{   //结构体可作为字段
        p1: Point2D,
        p2: Point2D,
    }
    ```

4. 运行测试

    ```rust
    fn main(){
        //实例化单元结构体
        let _nil = Nil;             
        
        //实例化元组结构体
        let pair = Pair(1, 0.1);    
        println!("pair contains {:?} and {:?}", pair.0, pair.1);
        //解构一个元组结构体
        let Pair(integer, decimal) = pair;
        //访问解构后实例的方法
        println!("pair contains {:?} and {:?}", Pair(integer,decimal).0, decimal);

        //实例化C语言风格结构体
        let my_point: Point2D = Point2D{x: 0.3, y: 0.4};
        println!("my_point.x = {}, my_point.y = {}", my_point.x, my_point.y);
        //用结构体更新语法创建新的point，这样可以用到之前的point字段
        let new_point = Point2D{y: 0.5, ..my_point};
        println!("new_point.x = {}, new_point.y = {}", new_point.x, new_point.y);
        //使用let来解构 my_point
        let Point2D{ x: my_x, y: my_y } = my_point;
        //上述结构体的实例化也是个表达式
        let _rectangle = Rectangle{
            p1: Point2D{ x: my_x, y: my_y },    //实例化的表达式
            p2: my_point,
        };
        println!("Rectangle is {:?}", _rectangle);  //fmt::Debug输出    
    }
    /* 输出结果
    pair contains 1 and 0.1
    pair contains 1 and 0.1
    my_point.x = 0.3, my_point.y = 0.4
    new_point.x = 0.3, new_point.y = 0.5
    Rectangle is Rectangle { p1: Point2D { x: 0.3, y: 0.4 }, p2: Point2D { x: 0.3, y: 0.4 } }
    */
    ```

### enums

1. 用enum创建枚举变量，其中的每个可能取值相互独立

    ```rust
    //枚举类型，各个可能取值相互独立
    enum WebEvent {
        PageLoad,
        PageUnload,             //单元结构体
        KeyPress(char),
        Paste(String),          //元组结构体
        Click{ x: i64, y: i64}  //普通结构体
    }
    ```

2. 类型别名
    + `type WebE = WebEvent;   //为WebEvent起别名WebE`
3. `match`依据enum的不同取值匹配不同动作

    ```rust
    //此函数将一个`WebEvent` enum作为参数，无返回值
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebE::Paste(s) => println!("pasted \"{}\".", s),
            WebE::Click{ x, y } => {
                println!("clicked at x = {}, y = {}", x, y);
            },
        }
    }
    ```

4. 运行测试

    ```rust
    //实例化 enum 枚举变量
    let load = WebE::PageLoad;
    let press = WebE::KeyPress('x');
    let paste = WebE::Paste("my paste text".to_owned());
    let click = WebE::Click{ x: 50, y: 100 };
    let unload = WebE::PageUnload;

    //根据枚举变量的不同取值执行不同动作
    inspect(load);
    inspect(press);
    inspect(paste);
    inspect(click);
    inspect(unload);
    /* 输出结果
    page loaded
    pressed 'x'.
    pasted "my paste text".
    clicked at x = 50, y = 100
    page unloaded
    */
    ```

#### operation枚举类的实现

```rust
enum DoSomeThingsForNumbers{
    Add,
    Subtract,
}
type Operation = DoSomeThingsForNumbers;    //类型别名
impl Operation{
    //self是常见的别名
    fn run(&self, x: i32, y: i32) -> i32 {  //返回i32
        match self{ //self为引用的参数的别名，等价Add或Subtract
            Self::Add => x + y,             //大写Self等价Operation
            Self::Subtract => x - y,
        }
    } 
}
fn main(){
    //实例化枚举类并调用imple的run方法进行计算
    let add = Operation::Add.run(1,2);
    let subtract = Operation::Subtract.run(2,1);
    println!("add(1,2) = {}", add);
    println!("subtract(2,1) = {}", subtract);
}
/* 输出结果
add(1,2) = 3
subtract(2,1) = 1
*/
```

#### use 的使用

```rust
enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Soldier,
}

fn main(){
    use Status::{ Poor, Rich }; //显式使模块（字段）可用
    use Work::*;                //使*全部模块（字段）可用
    
    let status = Poor;      //等价Status::Poor
    let work = Civilian;    //等价Work::Civilian
    
    match status{
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money......"),
    }
    match work{
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
```

#### implicit/explicit discriminator 枚举变量的显式和隐式辨别值

1. implicit discriminator

    ```rust
    // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
    enum Number {
        Zero,
        One,
        Two,
    }
    ```

2. explicit discriminator

    ```rust
    // 拥有显式辨别值（explicit discriminator）的 enum
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    ```

3. 测试运行

    ```rust
    //implicit/explicit discriminator
    fn main() {
        // `enum` 可以转成整型。
        println!("One is {}",Number::One as i32);
        println!("violets are #{:06x}",Color::Blue as i32);}
    }
    /* 输出结果
    One is 1
    violets are #0000ff
    */
    ```

#### enum 实例：链表的实现(linked-list)

A common use for enums is to create a linked-list:

```rust
/*
 * enum 实现linked-list 链表
 */
enum List {
    Cons(u32, Box<List>),   //元组结构体，表示链表的数据域和指针域
    Nil,    //末节点
}
impl List { //实现实现List的各种方法
    fn new() -> List{
        List::Nil
    }
    fn prepend(self, elem: u32) -> List{
        //前插法，返回的结点作头节点
        List::Cons(elem, Box::new(self))  //生成并返回一个List结点，其指针指向传入的结点
    }
    fn len(&self) -> u32 {
        match *self{
            //不能得到tail的所有权，因为self是借用的
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }       
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil")
            }
        }
    }
}
//实例化List
let mut list = List::new();
list = list.prepend(1);
list = list.prepend(2);
list = list.prepend(3);
println!("linked list has length: {}", list.len());
println!("{}", list.stringify());
/* 输出结果
linked list has length: 3
3, 2, 1, Nil
*/
```

### constants

1. const
    + `不可改变`的值（通常使用这个）
2. static
    + 具有 `static 生命周期`，可以是可变的变量（`static mut`）
    + static 生命周期
        + 数据保存在可执行文件的只读内存区中
        + static是可能的生命周期中最长的
        + 声明方法
            + `static NUM: i32 = 18;`
            + `let static_string = "I'm in read-only memory";`
                + string 等价于 &'static str
        + 离开作用域时引用无法再使用，但是数据仍然存在于二进制文件中
3. 运行测试

    ```rust
    //全局变量是在所有其他作用于之外声明的
    //'static 表示具有static生命周期
    //String 等价 &'static str ，String类型原生包含static生命周期
    static LANGUALE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD   //在一般函数中访问常量
    }
    fn main(){    
        //constants 常量
        let n = 16i32;
        println!("this is {}", LANGUALE);
        println!("the threshold is {}", THRESHOLD);
        println!("{} is {} than threshold", n, 
            if is_big(n) { "big" } else { "small" });
    }
    ```

## variable bindings 变量绑定

1. Values（值） (like literals) can be bound（绑定） to variables（变量）, using the `let`（关键字） binding.
    + `let an_integer = 1u32;`
2. 未使用的变量名称前添'_'可以消除编译警告
    + `let _unused_variable = 3u32;`

3. 运行测试

    ```rust
    fn main(){
        //变量绑定默认 immutable 不可变
        let an_integer = 1u32;
        //加上mut修饰后使变量可变
        let mut a_boolean = true;
        let unit = ();
        //拷贝值
        let copied_integer = an_integer;
        println!("An integer: {:?}", copied_integer);
        println!("A boolean: {:?}", a_boolean);
        println!("Meet the unit value: {:?}", unit);
        //编译器会对未使用的变量绑定产生警告
        //可以给变量名加上下划线'_'前缀来消除警告
        let _unused_variable = 3u32;
    }
    ```

### mutability 变量

1. let绑定变量默认不可变
    + `let an_integer = 1u32;`
2. 加上mut使可变
    + `let mut a_boolean = true;`
3. 运行测试

    ```rust
    fn main(){
        //变量绑定默认 immutable 不可变
        let an_integer = 1u32;
        // an_integer += 1 是错误的
        //加上mut修饰后使变量可变
        let mut a_boolean = true;
        println!("A boolean: {:?}", a_boolean);
        a_boolean = false;
        println!("A mut boolean: {:?}", a_boolean);
    }
    ```

### scope and shadowing 作用域和掩蔽

1. `变量绑定`有作用域`scope`，被限定在`{ }`包围的`block`代码块中`生存`和`使用`
2. 变量掩蔽
    + 由于不同作用域，允许同名变量的存在
3. 运行测试

    ```rust
    fn main(){
        //变量掩蔽
        println!("-------variable shadowing--------");
        //此绑定生存于main函数中
        let long_lived_binding = 1;
        {
            //这是一个block代码块，比main的作用域更小
            //虽然是同名变量，但此绑定掩蔽了外面的绑定
            let long_lived_binding = 666;
            println!("inner long is {}", long_lived_binding);
        }
        println!("outer long is {}", long_lived_binding);
    }
    ```

### decalre first 预先声明

1. 可以先`declare`声明，在后面再`initialize`初始化
2. 很少用，可能会导使用未初始化的变量

### freezing 冻结

当变量被相同名称变量绑定时，在当前作用域内会冻结（不可改变）

```rust
fn main(){
    //Freeze 冻结
    let mut _mutable_integer = 7i32;    //声明可变变量
    {
        //这是一个block代码块，比main的作用域小
        //冻结可变变量（在当前作用域内不再可变）
        let _mutable_integer = _mutable_integer;
        //但是加上mut后再次可变 =_= ，挺无聊的，还起名叫 freeze
        /*
        let mut _mutable_integer = _mutable_integer;
        _mutable_integer += 1;
        println!("surprise is {}",_mutable_integer);
        */
    }
    //go out of scope，Freeze over 冻结结束，变量依旧可变
    _mutable_integer += 1;
    println!("_mutable_integer is {}", _mutable_integer);
}
```

## types 类型

### casting 转换

1. `as`：显式类型转换
    + `let integer = decimal as u8;`
2. 不提供隐式类型转换
    + `let integer: u8 = decimal; //错误`
3. `#![allow(overflowing_literals)]`：不显示类型转换产生的溢出警告
4. 运行测试

    ```rust
    //不显示类型转换产生的溢出警告
    #![allow(overflowing_literals)]
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
    }
    ```

### literals 字面量

1. `let x = 1u8;    //后缀说明`
2. `let f: f32 = 1.0;    //常规说明`
3. `let n = 1;    //默认`

### inference 推断

```rust
fn main() {
    //inference 推断
    //elem类型为u8
    let elem = 5u8;
    //只知道vec某东西构成的向量Vec<_>，不知道具体类型
    let mut vec = Vec::new();
    //编译器推断得到vec为u8类型的向量
    vec.push(elem);
    println!("vec：{:?}", vec);
}
```

### aliasing 别名

1. `type`关键字起别名
2. 别名为驼峰式：如`CamelCase`（原生类型i8等例外）
    + `#![allow(non_camel_case_types)]`：允许非驼峰式起别名
3. 运行测试

    ```rust
    //允许非驼峰式起别名
    #![allow(non_camel_case_types)]

    //为u64起别名
    type Inch = u64;
    type u64_t = u64;   //宏允许非驼峰式别名
    
    fn main(){
        let inch: Inch = 2 as u64_t;
        println!("inch = {}", inch);
    }
    ```

## conversion 类型转换

Rust 使用 `trait` 解决类型之间的转换问题，最一般的转换会用到 `From` 和 `Into` 两个 trait

### From and Into

1. `From` 和 `Into` 两者内在联系，A能转换成B，那么B也能转换成A
2. 标准库已定义的 trait

    ```rust
    fn main(){
        let my_str = "hello";
        let my_string = String::from(my_str);   //标准库定义的From trait
    }
    ```

3. 自定义类型的From trait

    ```rust
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    //定义一个从i32类型转换成Number类型的trait
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    fn main(){
        let num = Number::from(666);
        println!("My number is {:?}", num); //fmt::Debug打印结构体Number
    }
    ```

### TryFrom and TryInto

1. 类似于From和Into，是通用的类型转换trait
2. 不同于 From/Into 的是，`TryFrom` 和 `TryInto` trait 用于易出错的转换，也正因如此，其返回值是 `Result` 型
3. 运行测试

    ```rust
    //导入TryForm， TryInto trait
    use std::convert::{ TryFrom, TryInto };

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32); //偶数，元组结构体（具名元组）

    //实现实现自定义结构的tryFrom tait，并为无法转换的结果设定'()'为返回值
    impl TryFrom<i32> for EvenNumber {
        type Error = ();    //将空元组起别名为Error
        //函数的返回结果集为 EvenNumber实例 或 ()（空元组）
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))   //如果为偶数，返回数值
            } else {
                Err(()) //如果为奇数，返回空元组
            }
        }
    }

    fn main(){
        //TryFrom
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));    
        //TryInto
        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));

    }
    ```

### ToString and FromStr

1. 将`任何类型`转换成`String`的方法
    + 实现 `ToString` trait（不推荐）
    + 实现 `fmt::Display` trait，会自动提供 `ToString` trait（**推荐**）

2. 将String转换成数字

## Expressions 表达式

```rust
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
//x is 5
//y is 155
//z is ()
```

## flowOfControl 控制流

### if-else 条件判断

```rust
fn main(){
    let n = 5;
    let big_n = 
        if n < 10 && n > -10{
            println!("返回一个i32的值");
            10 * n
        } else {
            println!("由于代码块最后一条表达式带分号，返回一个()");
            n / 2
        };   //let绑定需要分号结尾
    println!("n = {} -> big_n = {}", n, big_n);
}
```

### loop 循环 和 'label 标签

```rust
fn main(){
    let mut i = 0u32;
    //无限循环
    let result = 'outer: loop {
        println!("enter the outer loop"); 
        'inner: loop {  //'label记得跟：
            println!("enter the inner loop");
            i += 1;
            if i == 3 {
                println!("i is {}, ok, that's enough", i);
                break 'outer i * 10;
            }
        }
    };
    //break后跟loop的返回值
    println!("'outer: loop return {}", result);
}
```

### while 循环

### for 循环

1. `for in`遍历`Iterator`迭代器(用区间标记a...b，创建迭代器)
    + `for n in 1...101`
        + 1...101表示1,2,...,100，步长为1
        + 1...=101表示1,2,...,101，步长为1
2. 创建迭代器
    + 区间标记`a..b`
        + `for n in 1...101`
    + `iter()`：在每次迭代中借用集合中的一个元素
        + 集合中的元素本身不会改变，在循环互仍可使用
    + `into_iter()`
        + 会消耗集合，每次迭代中，会提供集合中的数据本身，并在循环后被`remove`
    + `iter_mut()`
        + `mutably`可变地借用集合中的每个元素，从而允许集合被就地修改

3. 运行测试

    ```rust
    fn main() {
        println!("======for in=====");
        let names = vec!["Bob", "Alice", "Dave"];
        for name in names.iter() {
            match name {
                //name 为 借用 的集合数据 &
                &"Alice" => println!("Oh! It's you, {}", name),
                _ => println!("Hello {}！", name),
            }
        }
        println!(":::::afer iter(), names: {:?}", names);
        
        for name in names.into_iter() {
            match name {
                //直接使用集合中的数据
                "Alice" => println!("Oh! It's you, {}", name),
                _ => println!("Hello {}！", name),
            }
        } 

        println!(":::::afer into_iter(), names was removed");

        let mut names = vec!["Bob", "Alice", "Dave"];
        for name in names.iter_mut() {
            //*name 修改原地址数据，match返回str作右值进行赋值
            *name = match name {
                &mut "Alice" => "lend from ownership",
                _ => "Changed", 
            }
        }
        println!(":::::afer iter_mut(), names: {:?}", names);
    }
    ```

### Match 匹配

```rust
fn main() {
    println!("=====match====="); 
    let num = 13;
    match num {
        1 => println!("num is one!"),
        2 | 3 | 5 => println!("num is 2 | 3 | 5"),
        13..=19 => println!("num is 13...19"),
        _ => println!("num is other number"),
    }
    let pair = (1,2);
    println!("match 解构");
    match pair {
        (0, y) => println!("(0, {})", y),
        (x, 0) => println!("({}, 0)", x),
        _ => println!("It doesn't matter what they are"),
    }
    println!("tell me about {:?}", pair);
    println!("match 解引用");
    let reference = &4i32;
    match reference {
        //匹配时解引用
        &val => println!("Got a value via destructing:{:?}", val),
    }
    //用ref创建引用
    let ref _is_a_reference = 4i32;
    match *_is_a_reference {
        //匹配前解引用
        val => println!("Got a value via destructing:{:?}", val),
    }

    let mut mut_value = 5i32;
    match mut_value {
        ref mut m => {
            //已经获得"mut_value"的引用，要先解引用才能改变值
            *m += 100;
            println!("We add 100, 'mut_value' = {:?}", m);
        }
    }

    println!("=====解构结构体=====");
    struct Foo{
        x: (u32, u32),
        y: u32
    }

    let foo = Foo{ x: (1, 2), y: 3 };
    let Foo{ x: (a, b), y } = foo;
    println!("a = {},b = {},c = {}", a, b, y);

    let Foo{ y: i, x: j } = foo;
    println!("i = {:?}, j= {:?}",i ,j);
    
    let Foo{ y, .. } = foo;
    println!("y = {}", y);
 
    println!("=====guard语句=====");
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("There are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation"),
    }

    println!("======match间接访问变量=====");
    fn age() -> u32{ 15 }   //age()返回u32    
    //匹配的age()，但是需要间接访问u32类型的返回值
    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm {} years old", n),
    }
    //匹配enum类型
    fn some_number() -> Option<u32> { Some(66) }
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting...{}", n),
        _ => (),
    }
}
```

### if let 和 while let

```rust
fn main() {
    println!("=====if let=====");
    //`if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    let number = Some(7);
    if let Some(i) = number {
        println!("Matched {:?}", i);
    } else {
        println!("Not Matched {:?}", i);
    }
    enum Color{
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    if let Color::RGB(red, green, blue) = color {
        println!("color matched ({:?}, {:?}, {:?})", red, green, blue);
    }
    println!("=====while let=====");
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 3 { 
            println!("Greater than 3, quit!");
            optional = None;
        } else .{
            println!("i is {:?}. Try again", i);
            optional = Some(i+1);
        }
    }
}
```

## 一些宏命令

1. `#![allow(dead_code)]`：隐藏对未使用代码的警告
2. `#![allow(overflowing_literals)]`：不显示类型转换产生的溢出警告
3. `#![allow(non_camel_case_types)]`：允许非驼峰式起别名
4. `#![allow(unused_labels)]`：允许未使用的标签
