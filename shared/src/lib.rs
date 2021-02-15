pub struct Test;

impl Test {
    pub fn new() -> Self {
        let _ = foo::foo();
        let _ = foo::bar();
        Self
    }
}
