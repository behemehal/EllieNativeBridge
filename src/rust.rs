use enum_as_inner::EnumAsInner;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PlatformArchitecture {
    B16,
    B32,
    B64,
}

#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub id: usize,
    pub stack_id: usize,
    pub frame_pos: usize,
    pub pos: usize,
    pub stack_caller: Option<usize>,
    pub arch: PlatformArchitecture,
}

#[derive(Clone, Debug)]
pub struct EllieInteger {
    pub as_isize: isize,
    pub as_usize: usize,
}

impl From<isize> for EllieInteger {
    fn from(value: isize) -> Self {
        EllieInteger {
            as_isize: value,
            as_usize: value as usize,
        }
    }
}

impl From<usize> for EllieInteger {
    fn from(value: usize) -> Self {
        EllieInteger {
            as_isize: value as isize,
            as_usize: value,
        }
    }
}

impl From<&EllieInteger> for isize {
    fn from(value: &EllieInteger) -> Self {
        value.as_isize
    }
}

impl From<&EllieInteger> for usize {
    fn from(value: &EllieInteger) -> Self {
        value.as_usize
    }
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum EllieData {
    Integer(EllieInteger),
    Float(f32),
    Double(f64),
    Byte(u8),
    Bool(bool),
    String(String),
    Char(char),
    Void,
    Null,
    Array(Vec<EllieData>),
    Class(Vec<EllieData>),
}

impl Into<EllieData> for &str {
    fn into(self) -> EllieData {
        EllieData::String(self.to_string())
    }
}

impl Into<EllieData> for String {
    fn into(self) -> EllieData {
        EllieData::String(self)
    }
}

impl Into<EllieData> for char {
    fn into(self) -> EllieData {
        EllieData::Char(self)
    }
}

impl Into<EllieData> for u8 {
    fn into(self) -> EllieData {
        EllieData::Byte(self)
    }
}

impl Into<EllieData> for f32 {
    fn into(self) -> EllieData {
        EllieData::Float(self)
    }
}

impl Into<EllieData> for f64 {
    fn into(self) -> EllieData {
        EllieData::Double(self)
    }
}

impl Into<EllieData> for isize {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for usize {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self,
        })
    }
}

#[cfg(target_pointer_width = "64")]
impl Into<EllieData> for i128 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

#[cfg(target_pointer_width = "64")]
impl Into<EllieData> for u128 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for u64 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for i64 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for u32 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for i32 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for u16 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for i16 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for bool {
    fn into(self) -> EllieData {
        EllieData::Bool(self)
    }
}

impl Into<EllieData> for () {
    fn into(self) -> EllieData {
        EllieData::Void
    }
}

impl Into<EllieData> for Vec<EllieData> {
    fn into(self) -> EllieData {
        EllieData::Array(self)
    }
}

#[derive(Clone, Debug)]
pub enum FunctionAnswer {
    Ok(EllieData),
    RuntimeError(String),
}

#[derive(Clone, Debug)]
pub struct FunctionCallParameter {
    pub data: EllieData,
    pub memory_location: usize,
}

pub type FunctionCallback =
    extern "Rust" fn(arguments: Vec<FunctionCallParameter>) -> FunctionAnswer;

pub struct EllieFunction {
    pub name: &'static str,
    pub on_call: FunctionCallback,
}

pub struct EllieModule {
    pub name: &'static str,
    pub version: &'static str,
    pub functions: Vec<EllieFunction>,
}
