#![allow(unused_labels)]
#![allow(dead_code)]

fn main() {
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


    println!("======loop=====");
    let mut i = 0u32;
    let result = 'outer: loop {
        println!("enter the outer loop"); 
        'inner: loop {  //
            println!("enter the inner loop");
            i += 1;
            if i == 3 {
                println!("i is {}, ok, that's enough", i);
                break 'outer i * 10;
            }
        }
    };
    assert_eq!(result, 30);
    println!("'outer: loop return {}", result);

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
        } else {
            println!("i is {:?}. Try again", i);
            optional = Some(i+1);
        }
    }
}