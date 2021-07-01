//导入TryForm， TryInto trait
use std::convert::{ TryFrom, TryInto };

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

#[derive(Debug, PartialEq)]
struct EvenNumber(i32); //偶数，元组结构体（具名元组）

//继承实现自定义结构的tryFrom tait，并为无法转换的结果设定'()'为返回值
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

struct Circle {
    radius: i32;
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius);
    }
}

fn main(){
    let my_str = "hello";
    let my_string = String::from(my_str);   //标准库定义的From trait
    println!("my_string is {}", my_string);

    let num = Number::from(666);
    println!("My number is {:?}", num);     //fmt::Debug打印结构体Number

    //TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));    
    //TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
