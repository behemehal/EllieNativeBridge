use enum_as_inner::EnumAsInner;
use std::any::Any;

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

/// Opaque heap handle for a `bytes` value living in the VM heap.
/// Native code never dereferences this; access goes through `EllieCtx`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EllieBytesHandle(pub u64);

impl EllieBytesHandle {
    pub fn id(&self) -> u64 {
        self.0
    }
}

/// Opaque heap handle for a generic native-owned resource (TcpStream,
/// UdpSocket, File, sqlite3*, ...). The VM owns the Box and runs the
/// destructor when the handle is explicitly dropped or the heap entry
/// is freed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EllieHandle(pub u64);

impl EllieHandle {
    pub fn id(&self) -> u64 {
        self.0
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
    /// Reference to a mutable byte buffer in the VM heap. Read/write through `EllieCtx`.
    Bytes(EllieBytesHandle),
    /// Reference to a generic native-owned resource. Access via `EllieCtx::handle_*`.
    Handle(EllieHandle),
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

/// ABI version this bridge implements. The VM rejects modules whose
/// `abi_version` doesn't match. Bump on breaking changes to `EllieData`,
/// `EllieCtx`, or `FunctionCallback`.
pub const ELLIE_RUST_ABI_VERSION: u32 = 1;

/// Per-call VM context. The loader supplies a concrete implementation that
/// borrows the VM heap; native code uses the trait methods to read/write
/// heap-resident byte buffers without copies.
///
/// References returned by `bytes_mut_slice` / `bytes_view` are only valid
/// until the next mutating call on this ctx (which may resize the underlying
/// vec). Don't store them across calls.
pub trait EllieCtx {
    fn bytes_new(&mut self, data: Vec<u8>) -> EllieBytesHandle;
    fn bytes_len(&self, h: EllieBytesHandle) -> usize;
    fn bytes_pos(&self, h: EllieBytesHandle) -> usize;
    fn bytes_set_pos(&mut self, h: EllieBytesHandle, pos: usize);
    fn bytes_get(&self, h: EllieBytesHandle, i: usize) -> Option<u8>;
    fn bytes_set(&mut self, h: EllieBytesHandle, i: usize, v: u8) -> bool;
    fn bytes_view(&self, h: EllieBytesHandle) -> Option<&[u8]>;
    fn bytes_clone(&self, h: EllieBytesHandle) -> Option<Vec<u8>>;
    fn bytes_mut_slice(
        &mut self,
        h: EllieBytesHandle,
        offset: usize,
        len: usize,
    ) -> Option<&mut [u8]>;
    fn bytes_append(&mut self, h: EllieBytesHandle, src: &[u8]) -> bool;

    // ── Generic native-resource handles ────────────────────────────────────
    //
    // Trait-object compatibility forbids generic methods, so `handle_new`
    // takes a pre-boxed `Box<dyn Any + Send + Sync>`. Accessors return
    // `&dyn Any` and callers downcast.
    //
    // Lifetime: the resource lives until `handle_drop` is called or the
    // heap entry is freed. Today the VM has no GC, so dropped-only is the
    // path that actually closes sockets/files.

    fn handle_new(&mut self, value: Box<dyn Any + Send + Sync>) -> EllieHandle;
    fn handle_view(&self, h: EllieHandle) -> Option<&(dyn Any + Send + Sync)>;
    fn handle_view_mut(&mut self, h: EllieHandle) -> Option<&mut (dyn Any + Send + Sync)>;
    fn handle_drop(&mut self, h: EllieHandle) -> bool;
}

pub type FunctionCallback = extern "Rust" fn(
    ctx: &mut dyn EllieCtx,
    arguments: Vec<FunctionCallParameter>,
) -> FunctionAnswer;

pub struct EllieFunction {
    pub name: &'static str,
    pub on_call: FunctionCallback,
}

pub struct EllieModule {
    pub abi_version: u32,
    pub name: &'static str,
    pub version: &'static str,
    pub functions: Vec<EllieFunction>,
}
