mod r#static {
    use js_sys::{ArrayBuffer, Uint8Array};
    use node_sys::Buffer;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn alloc() {
        let size = Default::default();
        let fill = Buffer::from_array(Box::new([]));
        let encoding = Default::default();
        Buffer::alloc(size, Some(&fill), encoding);
    }

    #[wasm_bindgen_test]
    fn alloc_unsafe() {
        let size = Default::default();
        Buffer::alloc_unsafe(size);
    }

    #[wasm_bindgen_test]
    fn byte_length() {
        let string = &"".into();
        let encoding = Default::default();
        Buffer::byte_length(&string, encoding);
    }

    #[wasm_bindgen_test]
    fn compare_() {
        let buf = Buffer::from_array(Box::new([]));
        Buffer::compare_(&buf, &buf);
    }

    #[wasm_bindgen_test]
    fn concat() {
        let list = Box::new([]);
        let total_length = Default::default();
        Buffer::concat(list, total_length);
    }

    #[wasm_bindgen_test]
    fn from_array() {
        let array = Box::new([]);
        Buffer::from_array(array);
    }

    #[wasm_bindgen_test]
    fn from_array_buffer() {
        let buffer = ArrayBuffer::new(0);
        let byte_offset = Default::default();
        let length = Default::default();
        Buffer::from_array_buffer(&buffer, byte_offset, length);
    }

    #[wasm_bindgen_test]
    fn from_array_uint8() {
        let array = Uint8Array::new_with_length(0);
        Buffer::from_array_uint8(&array);
    }

    #[wasm_bindgen_test]
    fn from_string() {
        let string = &"".into();
        let encoding = Default::default();
        Buffer::from_string(string, encoding);
    }

    #[wasm_bindgen_test]
    fn is_buffer() {
        assert!(Buffer::is_buffer(&Buffer::from_array(Box::new([]))));
    }

    #[wasm_bindgen_test]
    fn is_encoding() {
        assert!(Buffer::is_encoding(&"utf8".into()));
    }

    #[wasm_bindgen_test]
    fn pool_size() {
        Buffer::pool_size();
    }
}

mod instance {
    use node_sys::Buffer;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn compare() {
        let buffer = crate::helper::buffer::new();
        let target = &buffer;
        let target_start = Default::default();
        let target_end = Default::default();
        let source_start = Default::default();
        let source_end = Default::default();
        buffer.compare(target, target_start, target_end, source_start, source_end);
    }

    #[wasm_bindgen_test]
    fn copy() {
        let buffer = crate::helper::buffer::new();
        let target = crate::helper::buffer::new();
        let target_start = Default::default();
        let target_end = Default::default();
        let source_start = Default::default();
        let source_end = Default::default();
        buffer.compare(&target, target_start, target_end, source_start, source_end);
    }

    #[wasm_bindgen_test]
    fn entries() {
        let buffer = crate::helper::buffer::new();
        buffer.entries();
    }

    #[wasm_bindgen_test]
    fn equals() {
        let buffer = crate::helper::buffer::new();
        buffer.equals(&buffer);
    }

    #[wasm_bindgen_test]
    fn fill() {
        let buffer = crate::helper::buffer::new();
        let value = &0.into();
        let offset = Default::default();
        let end = Default::default();
        let encoding = Default::default();
        buffer.fill(value, offset, end, encoding);
    }

    #[wasm_bindgen_test]
    fn includes() {
        let buffer = crate::helper::buffer::new();
        let value = &0.into();
        let offset = Default::default();
        let encoding = Default::default();
        buffer.includes(value, offset, encoding);
    }

    #[wasm_bindgen_test]
    fn index_of() {
        let buffer = crate::helper::buffer::new();
        let value = &0.into();
        let offset = Default::default();
        let encoding = Default::default();
        buffer.index_of(value, offset, encoding);
    }

    #[wasm_bindgen_test]
    fn keys() {
        let buffer = crate::helper::buffer::new();
        buffer.keys();
    }

    #[wasm_bindgen_test]
    fn last_index_of() {
        let buffer = crate::helper::buffer::new();
        let value = &0.into();
        let offset = Default::default();
        let encoding = Default::default();
        buffer.last_index_of(value, offset, encoding);
    }

    #[wasm_bindgen_test]
    fn length() {
        let buffer = crate::helper::buffer::new();
        buffer.length();
    }

    #[wasm_bindgen_test]
    fn read_big_int64_be() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let offset = None;
        buffer.read_big_int64_be(offset);
    }

    #[wasm_bindgen_test]
    fn read_big_int64_le() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let offset = None;
        buffer.read_big_int64_le(offset);
    }

    #[wasm_bindgen_test]
    fn read_big_uint64_be() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let offset = None;
        buffer.read_big_uint64_be(offset);
    }

    #[wasm_bindgen_test]
    fn read_big_uint64_le() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let offset = None;
        buffer.read_big_uint64_le(offset);
    }

    #[wasm_bindgen_test]
    fn read_double_be() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let offset = None;
        buffer.read_double_be(offset);
    }

    #[wasm_bindgen_test]
    fn read_double_le() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let offset = None;
        buffer.read_double_le(offset);
    }

    #[wasm_bindgen_test]
    fn read_float_be() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let offset = None;
        buffer.read_float_be(offset);
    }

    #[wasm_bindgen_test]
    fn read_float_le() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let offset = None;
        buffer.read_float_le(offset);
    }

    #[wasm_bindgen_test]
    fn read_int_be() {
        let buffer = Buffer::alloc(6.into(), None, None);
        let offset = 0.into();
        let length = 6;
        buffer.read_int_be(offset, length);
    }

    #[wasm_bindgen_test]
    fn read_int_le() {
        let buffer = Buffer::alloc(6.into(), None, None);
        let offset = 0.into();
        let length = 6;
        buffer.read_int_le(offset, length);
    }

    #[wasm_bindgen_test]
    fn read_uint_be() {
        let buffer = Buffer::alloc(6.into(), None, None);
        let offset = 0.into();
        let length = 6;
        buffer.read_uint_be(offset, length);
    }

    #[wasm_bindgen_test]
    fn read_uint_le() {
        let buffer = Buffer::alloc(6.into(), None, None);
        let offset = 0.into();
        let length = 6;
        buffer.read_uint_le(offset, length);
    }

    #[wasm_bindgen_test]
    fn read_int8() {
        let buffer = Buffer::alloc(1.into(), None, None);
        let offset = None;
        buffer.read_int8(offset);
    }

    #[wasm_bindgen_test]
    fn read_int16_be() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let offset = None;
        buffer.read_int16_be(offset);
    }

    #[wasm_bindgen_test]
    fn read_int16_le() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let offset = None;
        buffer.read_int16_le(offset);
    }

    #[wasm_bindgen_test]
    fn read_uint8() {
        let buffer = Buffer::alloc(1.into(), None, None);
        let offset = None;
        buffer.read_uint8(offset);
    }

    #[wasm_bindgen_test]
    fn read_uint16_be() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let offset = None;
        buffer.read_uint16_be(offset);
    }

    #[wasm_bindgen_test]
    fn read_uint16_le() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let offset = None;
        buffer.read_uint16_le(offset);
    }

    #[wasm_bindgen_test]
    fn read_uint32_be() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let offset = None;
        buffer.read_uint32_be(offset);
    }

    #[wasm_bindgen_test]
    fn read_uint32_le() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let offset = None;
        buffer.read_uint32_le(offset);
    }

    #[wasm_bindgen_test]
    fn slice() {
        let buffer = Buffer::alloc(1.into(), None, None);
        let start = None;
        let end = None;
        buffer.slice(start, end);
    }

    #[wasm_bindgen_test]
    fn subarray() {
        let buffer = Buffer::alloc(1.into(), None, None);
        let start = None;
        let end = None;
        buffer.subarray(start, end);
    }

    #[wasm_bindgen_test]
    fn swap16() {
        let buffer = Buffer::alloc(2.into(), None, None);
        buffer.swap16();
    }

    #[wasm_bindgen_test]
    fn swap32() {
        let buffer = Buffer::alloc(4.into(), None, None);
        buffer.swap32();
    }

    #[wasm_bindgen_test]
    fn swap64() {
        let buffer = Buffer::alloc(8.into(), None, None);
        buffer.swap64();
    }

    #[wasm_bindgen_test]
    fn to_json() {
        let buffer = crate::helper::buffer::new();
        buffer.to_json();
    }

    #[wasm_bindgen_test]
    fn to_string() {
        let buffer = crate::helper::buffer::new();
        let encoding = None;
        let start = None;
        let end = None;
        buffer.to_string(encoding, start, end);
    }

    #[wasm_bindgen_test]
    fn values() {
        let buffer = crate::helper::buffer::new();
        buffer.values();
    }

    #[wasm_bindgen_test]
    fn write() {
        let buffer = crate::helper::buffer::new();
        let string = &"".into();
        let offset = Default::default();
        let length = Default::default();
        let encoding = Default::default();
        buffer.write(string, offset, length, encoding);
    }

    #[wasm_bindgen_test]
    fn write_big_int64_be() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let value = 0i64;
        let offset = Default::default();
        buffer.write_big_int64_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_big_int64_le() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let value = 0i64;
        let offset = Default::default();
        buffer.write_big_int64_le(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_int8() {
        let buffer = Buffer::alloc(1.into(), None, None);
        let value = 0i8;
        let offset = Default::default();
        buffer.write_int8(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_int16_be() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let value = 0i16;
        let offset = Default::default();
        buffer.write_int16_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_int16_le() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let value = 0i16;
        let offset = Default::default();
        buffer.write_int16_le(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_int32_be() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let value = 0i32;
        let offset = Default::default();
        buffer.write_int32_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_int32_le() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let value = 0i32;
        let offset = Default::default();
        buffer.write_int32_le(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_big_uint64_be() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let value = 0u64;
        let offset = Default::default();
        buffer.write_big_uint64_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_big_uint64_le() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let value = 0u64;
        let offset = Default::default();
        buffer.write_big_uint64_le(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_double_be() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let value = 0f64;
        let offset = Default::default();
        buffer.write_double_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_double_le() {
        let buffer = Buffer::alloc(8.into(), None, None);
        let value = 0f64;
        let offset = Default::default();
        buffer.write_double_le(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_float_be() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let value = 0f32;
        let offset = Default::default();
        buffer.write_float_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_float_le() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let value = 0f32;
        let offset = Default::default();
        buffer.write_float_le(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_uint8() {
        let buffer = Buffer::alloc(1.into(), None, None);
        let value = 0u8;
        let offset = Default::default();
        buffer.write_uint8(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_uint16_be() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let value = 0u16;
        let offset = Default::default();
        buffer.write_uint16_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_uint16_le() {
        let buffer = Buffer::alloc(2.into(), None, None);
        let value = 0u16;
        let offset = Default::default();
        buffer.write_uint16_le(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_uint32_be() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let value = 0u32;
        let offset = Default::default();
        buffer.write_uint32_be(value, offset);
    }

    #[wasm_bindgen_test]
    fn write_uint32_le() {
        let buffer = Buffer::alloc(4.into(), None, None);
        let value = 0u32;
        let offset = Default::default();
        buffer.write_uint32_le(value, offset);
    }
}
