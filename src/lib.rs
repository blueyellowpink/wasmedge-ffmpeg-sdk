mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

use anyhow;

pub fn add(a: u32, b: u32) -> u32 {
    unsafe { generated::add(a, b) }
}

pub fn subtract(a: u32, b: u32) -> u32 {
    unsafe { generated::subtract(a, b) }
}

pub type Pointer = generated::Pointer;

pub fn avformat_open_input(
    avformat_context: &mut AvFormatContext,
    file_name: &str,
) -> anyhow::Result<()> {
    unsafe {
        let ret = generated::avformat_open_input(
            avformat_context.as_ptr(),
            file_name.as_ptr(),
            file_name.len(),
        );
        if ret != 0u32 {
            return Err(anyhow::anyhow!("ERROR open input"));
        }
        Ok(())
    }
}

pub fn avformat_find_stream_info(avformat_context: &mut AvFormatContext) -> anyhow::Result<()> {
    unsafe {
        let ret = generated::avformat_find_stream_info(avformat_context.as_ptr());
        if ret != 0u32 {
            return Err(anyhow::anyhow!("ERROR open input"));
        }
        Ok(())
    }
}

pub fn avformat_alloc_output_context2(
    avformat_context: &mut AvFormatContext,
    file_name: &str,
) -> anyhow::Result<()> {
    unsafe {
        let ret = generated::avformat_alloc_output_context2(
            avformat_context.as_ptr(),
            file_name.as_ptr(),
            file_name.len(),
        );
        if ret != 0u32 {
            return Err(anyhow::anyhow!("ERROR alloc output context2"));
        }
        Ok(())
    }
}

pub struct AvFormatContext {
    ptr: Pointer,
}

impl AvFormatContext {
    pub fn new() -> Self {
        Self {
            ptr: Self::alloc_context(),
        }
    }

    pub fn get_context(&self) -> u64 {
        unsafe { generated::avformat_get_context(self.ptr) }
    }

    pub fn nb_streams(&self) -> u32 {
        unsafe { generated::avformat_get_nb_streams(self.ptr) }
    }

    pub fn streams(&self, index: u32) -> AvStream {
        unsafe {
            AvStream {
                ptr: generated::avformat_get_stream(self.ptr, index),
            }
        }
    }

    pub fn as_ptr(&self) -> Pointer {
        self.ptr
    }
}

impl AvFormatContext {
    fn alloc_context() -> Pointer {
        unsafe { generated::avformat_alloc_context() }
    }

    fn free_context(avformat_context: Pointer) -> () {
        unsafe { generated::avformat_free_context(avformat_context) }
    }
}

impl Drop for AvFormatContext {
    fn drop(&mut self) {
        Self::free_context(self.ptr);
    }
}

pub struct AvStream {
    ptr: Pointer,
}

impl AvStream {
    pub fn codecpar(&self) -> AvCodecParameters {
        todo!()
    }

    pub fn as_ptr(&self) -> Pointer {
        self.ptr
    }
}

pub struct AvCodecParameters {
    ptr: Pointer,
}

impl AvCodecParameters {
    pub fn codec_type(&self) -> AvMediaType {
        todo!()
    }
}

pub enum AvMediaType {
    UNKNOWN = -1,
    ///< Usually treated as AVMEDIA_TYPE_DATA
    VIDEO,
    AUDIO,
    DATA,
    ///< Opaque data information usually continuous
    SUBTITLE,
    ATTACHMENT,
    ///< Opaque data information usually sparse
    NB,
}
