use std::any::Any;
use std::collections::HashMap;

struct MyClass;
impl MyClass {
    fn print(&self) {
        println!("MyClass prints!");
    }
}

struct TypeErasedMap<'a> {
    storage: HashMap<String, &'a dyn Any>,
}

impl<'a> TypeErasedMap<'a> {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    fn store<T: Any>(&mut self, name: &str, obj: &'a T) {
        self.storage.insert(name.to_string(), obj);
    }

    fn get<T: Any>(&self, name: &str) -> Option<&T> {
        self.storage
            .get(name)
            .and_then(|any| any.downcast_ref::<T>())
    }
}

fn main() {
    let mut map = TypeErasedMap::new();
    let mc = MyClass;

    map.store("MyClass", &mc);

    if let Some(ref_mc) = map.get::<MyClass>("MyClass") {
        ref_mc.print();
    }
}
