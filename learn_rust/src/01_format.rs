/*
* Author：RJman
* email：rjman@foxmail.com
*/
use std::fmt;   //导入fmt，使fmt::Display可用

// 推导 `Structure` 的 `fmt::Debug` 实现
//否则显示 Structure` cannot be formatted using `{:?}`
#[derive(Debug)]
struct Structure(i32);  //包含单个 `i32` 的结构体。

#[derive(Debug)]    //使 `Deep` 也能够打印
struct Deep(Structure); //将 `Structure` 放到结构体 `Deep` 中

#[derive(Debug)]    //为MinMax实现fmt::Debug
struct MinMax(i64,i64);

impl fmt::Display for MinMax{   //为MinMax实现fmt::Display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        //使用self.number表示各个数据
        write!(f,"({},{})",self.0,self.1)
    }
}

#[derive(Debug)]    //为Point2D实现fmt::Debug
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{  //为Point2D实现fmt::Display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "(x = {}, y = {})", self.x, self.y)
    }
}

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

#[derive(Debug)]        //为List实现fmt::Debug
struct List(Vec<i32>);  //定义包含单个 `Vec` 的结构体 `List`

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
#[derive(Debug)]
struct City{
    name: &'static str, //城市名称
    lat: f32,   //纬度
    lon: f32,   //经度
}

impl fmt::Display for City{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        //write!和format类似，将格式化字符串写入缓冲区 f
        write!(f, "{}: {}°{}, {}°{}", self.name,
            self.lat, lat_c, self.lon, lon_c)
    }
}

#[derive(Debug)]
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "RGB({},{},{}) 0x{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue,
            self.red, self.green, self.blue)
    }
}

fn main(){
    println!("hello world!"/* 就是这么随意 */);
    //std::fmt 包含多种 traits
    
    println!("{name} {age}",name = "RJman",age = 21);     //命名参数
    println!("{0} {2} {1}", 0, 2, 'a');                     //指定参数
    println!("{:?}",(2,3));                                 //打印元组
    let pi = 3.1415926;
    println!(".N 表示精度：Pi is roughtly {0:>.3}",pi);         //.N表示精度
    println!(".N$ 表示精度：Pi is {:.1$}", pi, 5);      //.N$表示精度
    println!(".* 表示精度：Pi is {:.*}", 6, pi);        //.*表示精度
    println!("Pi is roughtly {:<04}， 0.5 is {:^04} of {:>08b}", pi, 1, 2);   //:04b 占4位其余补零，b指示为二进制
    
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
    //复数Complex结构的格式化输出
    let complex = Complex{re: 1.2, im: 3.4};
    println!("Debug print Complex：{:?}",complex);
    println!("Display print Complex：{}",complex);

    let v = List(vec![1,2,3]);  //vec!宏用来初始化一个vector
    println!("Debug List print：{:?}", v);
    println!("Display List print：{}", v);

    //Color和City的输出
    let city = City{ name: "HangZhou", lat: 30.0, lon: 120.0 };
    println!("Debug City print：{:?}", city);
    println!("Display City print：{}", city);
    let color = Color{ red: 223, green: 52, blue: 77 };
    println!("Debug Color print:{:?}", color);
    println!("Display Color print:{}", color);
    println!("----------loop_city----------");
    for city in [
        City{ name: "Dublin", lat: 53.347778, lon: -6.259772 },
        City{ name: "Oslo", lat: 59.95, lon: 10.75 },
        City{ name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("Debug City print：{:?}", *city);
        println!("Display City print：{}", *city);
    }
    println!("----------loop_color----------");
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("Debug Color print：{:?}", *color);
        println!("Display Color print：{}", *color);
    }
    println!("------------------------------");
}

