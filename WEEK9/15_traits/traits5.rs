trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// Fixed signature
fn some_func(item: &(impl SomeTrait + OtherTrait)) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    let s1 = SomeStruct;
    let s2 = OtherStruct;
    println!("SomeStruct result: {}", some_func(&s1));
    println!("OtherStruct result: {}", some_func(&s2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(&SomeStruct));
        assert!(some_func(&OtherStruct));
    }
}