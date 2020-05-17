#[derive(Clone)]
pub struct SharedMemory {
    data: String
}

// impl Clone for SharedMemory {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

impl SharedMemory {
    pub fn new(data: String) -> Self {
        SharedMemory {
            data: data
        }
    }
}