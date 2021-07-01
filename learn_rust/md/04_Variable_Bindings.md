# Rust

## Variable Bindings 变量绑定

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

### Mutability 变量

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

### Scope and Shadowing 作用域和掩蔽

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

### Decalre first 预先声明

1. 可以先`declare`声明，在后面再`initialize`初始化
2. 很少用，可能会导使用未初始化的变量

### Freezing 冻结

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
