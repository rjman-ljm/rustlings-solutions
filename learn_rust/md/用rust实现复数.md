# Rust实现Complex复数

## 结构体

```rust

#[derive(Debug)]    //为Point2D实现fmt::Debug
struct Complex{ //定义复数结构（re+im*i)
    re: f64,
    im: f64,
}

impl fmt::Display for Complex{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} + {}i", self.re, self.im)
    }
}
```

## 输出

```rust
let complex = Complex{re: 1.2, im: 3.4};
println!("Debug print Complex：{:?}",complex);
println!("Display print Complex：{}",complex);
```
