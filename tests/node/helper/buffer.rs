use node_sys::Buffer;

pub fn new() -> Buffer {
    Buffer::from_array(Box::new([]))
}
