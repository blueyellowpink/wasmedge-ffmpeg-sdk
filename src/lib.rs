mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

pub fn add(a: u32, b: u32) -> u32 {
    unsafe { generated::add(a, b) }
}

pub fn subtract(a: u32, b: u32) -> u32 {
    unsafe { generated::subtract(a, b) }
}

pub type Pointer = generated::Pointer;

pub struct AvFormat;

impl AvFormat {
    pub fn alloc_context() -> Pointer {
        unsafe { generated::avformat_alloc_context() }
    }

    pub fn get_context(ptr: Pointer) -> u64 {
        unsafe { generated::avformat_get_context(ptr) }
    }

    pub fn open_input(avformat_context: Pointer, file_name: &str) {
        unsafe {
            generated::avformat_open_input(avformat_context, file_name.as_ptr(), file_name.len());
        }
    }
}
