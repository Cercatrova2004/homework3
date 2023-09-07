use std::sync::Arc;
use std::ops::Deref;

struct Mystruct {
    data: i32,
}

struct Myrc {
    shared_data: Arc<Mystruct>,
}

impl Myrc {
    fn new(data: i32) -> Self {
        Myrc {
            shared_data: Arc::new(Mystruct { data }),
        }
    }
}

impl Clone for Myrc {
    fn clone(&self) -> Self {
        Myrc {
            shared_data: Arc::clone(&self.shared_data),
        }
    }
}

impl Deref for Myrc {
    type Target = Mystruct;

    fn deref(&self) -> &Self::Target {
        &*self.shared_data
    }
}

fn main() {
    let ptr1 = Myrc::new(97);
    let ptr2 = ptr1.clone();
    println!("Data: {}", ptr1.data);
    println!("Data: {}", ptr2.data);
}