# Rust

## Custom Types 自定义类型

1. 关键字`struct`：定义一个结构体（`structure`）
2. 关键字`enum`：定义一个枚举类型（`enumeration`）
3. 常量`constants`可以通过关键字`const`和`static`创建

### Structures

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

### Enums

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

#### Operation枚举类的实现

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
impl List { //继承实现List的各种方法
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

### Constants

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
