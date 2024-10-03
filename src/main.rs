use rust_smartpointers_own_smartpointer::MyBox;
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}

fn hello(name: &str){
    println!("Hello, {name}!");
}

#[cfg(test)]
mod tests{
    use std::ops::Deref;

    use rust_smartpointers_own_smartpointer::MyBox;


    #[test]
    fn compare_number(){
        let x = 5;
        let y = &x;
        let w = Box::new(x);
        let z = MyBox::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *w);
        assert_eq!(5, *z);
        assert_eq!(5, *(z.deref()));
    }
}