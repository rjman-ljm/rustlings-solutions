# Rust

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
        impl fmt::Display for Structure {   //implement 继承
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
