use libc::c_char;

#[repr(C)]
pub struct IntegerType {
    pub value: usize,
}

#[repr(C)]
pub struct StringType {
    pub value: *mut c_char,
}

#[repr(C)]
pub struct CharType {
    pub value: c_char,
}

#[repr(C)]
pub struct BooleanType {
    pub value: bool,
}

#[repr(C)]
pub struct FloatType {
    pub value: f64,
}

#[repr(C)]
pub struct DoubleType {
    pub value: f32,
}

#[repr(C)]
pub struct ByteType {
    pub value: u8,
}

#[repr(C)]
pub struct ArrayType {
    pub value: *mut Types,
}

#[repr(C)]
pub enum Types {
    Integer(IntegerType),
    String(StringType),
    Boolean(BooleanType),
    Float(FloatType),
    Double(DoubleType),
    Byte(ByteType),
    Array(ArrayType),
}
