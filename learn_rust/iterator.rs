//trait Iterator {
//    type Item;
//    fn next(mut self) -> Option<Self::Item>;
//}

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
//    for val in v1_iter {
//        println!("val = {}", val);
//    }

    while let Some(v) = v1_iter.next() {
        println!("while let val = {}", v);
    }

}
