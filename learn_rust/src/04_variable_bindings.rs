fn main(){
    let unit = ();
    //变量绑定默认 immutable 不可变
    let an_integer = 1u32;
    //加上mut修饰后使变量可变
    let mut a_boolean = true;

    //拷贝值
    let copied_integer = an_integer;
    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    a_boolean = false;
    println!("A mut boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
    //编译器会对未使用的变量绑定产生警告
    //可以给变量名加上下划线'_'前缀来消除警告
    let _unused_variable = 3u32;

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

    //Freeze 冻结
    let mut _mutable_integer = 7i32;    //声明可变变量
    {
        //这是一个block代码块，比main的作用域小
        //冻结可变变量（在当前作用域内不再可变）
        let _mutable_integer = _mutable_integer;
        //但是加上mut后再次可变 =_=，挺无聊的，还起名叫 freeze
        let mut _mutable_integer = _mutable_integer;
        _mutable_integer += 1;
        println!("surprise is {}",_mutable_integer);
    }
    //go out of scope，Freeze over 冻结结束，变量依旧可变
    _mutable_integer += 1;
    println!("_mutable_integer is {}", _mutable_integer);
}