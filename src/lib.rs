mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

use anyhow;

pub type Pointer = generated::Pointer;

pub fn av_read_frame(
    avformat_context: &AvFormatContext,
    avpacket: &AvPacket,
) -> anyhow::Result<()> {
    let ret = unsafe { generated::av_read_frame(avformat_context.as_ptr(), avpacket.as_ptr()) };
    if ret != 0 {
        return Err(anyhow::anyhow!("ERROR av read frame"));
    }
    Ok(())
}

pub fn av_packet_unref(avpacket: &AvPacket) {
    unsafe { generated::av_packet_unref(avpacket.as_ptr()) }
}

pub fn copy_packet(
    input_context: &AvFormatContext,
    output_context: &AvFormatContext,
    avpacket: &AvPacket,
    streams_list_index: usize,
) {
    unsafe {
        generated::copy_packet(
            input_context.as_ptr(),
            output_context.as_ptr(),
            avpacket.as_ptr(),
            streams_list_index as u32,
        )
    }
}

pub fn avformat_new_stream(avformat_context: &mut AvFormatContext) -> anyhow::Result<AvStream> {
    let ptr = unsafe { generated::avformat_new_stream(avformat_context.as_ptr()) };
    if ptr == 0u64 {
        return Err(anyhow::anyhow!("ERROR new stream"));
    }
    Ok(AvStream { ptr })
}

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
    avformat_context: &AvFormatContext,
    file_name: &str,
) -> anyhow::Result<AvFormatContext> {
    let ret = unsafe {
        generated::avformat_alloc_output_context2(
            avformat_context.as_ptr(),
            file_name.as_ptr(),
            file_name.len(),
        )
    };
    if ret == 1u64 {
        return Err(anyhow::anyhow!("ERROR alloc output context2"));
    }
    Ok(AvFormatContext { ptr: ret })
}

pub fn avformat_write_header(avformat_context: &mut AvFormatContext) -> anyhow::Result<()> {
    let ret = unsafe { generated::avformat_write_header(avformat_context.as_ptr()) };
    if ret != 0u32 {
        return Err(anyhow::anyhow!("ERROR avformat write header"));
    }
    Ok(())
}

pub fn avcodec_params_copy(dst: &AvCodecParameters, src: &AvCodecParameters) -> anyhow::Result<()> {
    let ret = unsafe { generated::avcodec_params_copy(dst.as_ptr(), src.as_ptr()) };
    if ret != 0 {
        return Err(anyhow::anyhow!("ERROR avcodec params copy"));
    }
    Ok(())
}

pub fn av_dump_format(
    avformat_context: &AvFormatContext,
    index: usize,
    file_name: &str,
    is_output: bool,
) {
    unsafe {
        generated::av_dump_format(
            avformat_context.as_ptr(),
            index as u32,
            file_name.as_ptr(),
            file_name.len(),
            is_output as u32,
        )
    }
}

pub fn avio_open(avformat_context: &AvFormatContext, file_name: &str) -> anyhow::Result<()> {
    let ret = unsafe {
        generated::avio_open(
            avformat_context.as_ptr(),
            file_name.as_ptr(),
            file_name.len(),
        )
    };
    if ret != 0 {
        return Err(anyhow::anyhow!("ERROR avio open"));
    }
    Ok(())
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

    pub fn streams(&self, index: usize) -> AvStream {
        unsafe {
            AvStream {
                ptr: generated::avformat_get_stream(self.ptr, index as u32),
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

// impl Drop for AvFormatContext {
//     fn drop(&mut self) {
//         Self::free_context(self.ptr);
//     }
// }

pub struct AvStream {
    ptr: Pointer,
}

impl AvStream {
    pub fn codecpar(&self) -> AvCodecParameters {
        unsafe {
            AvCodecParameters {
                ptr: generated::avstream_codecpar(self.ptr),
            }
        }
    }

    pub fn as_ptr(&self) -> Pointer {
        self.ptr
    }
}

pub struct AvCodecParameters {
    ptr: Pointer,
}

impl AvCodecParameters {
    pub fn codec_type(&self) -> Option<AvMediaType> {
        let codec_type = unsafe { generated::avcodec_params_codec_type(self.ptr) };
        match codec_type {
            0u32 => Some(AvMediaType::UNKNOWN),
            1u32 => Some(AvMediaType::VIDEO),
            2u32 => Some(AvMediaType::AUDIO),
            3u32 => Some(AvMediaType::DATA),
            4u32 => Some(AvMediaType::SUBTITLE),
            5u32 => Some(AvMediaType::ATTACHMENT),
            6u32 => Some(AvMediaType::NB),
            _ => None,
        }
    }

    pub fn as_ptr(&self) -> Pointer {
        self.ptr
    }
}

#[derive(PartialEq)]
pub enum AvMediaType {
    UNKNOWN = 0,
    VIDEO,
    AUDIO,
    DATA,
    SUBTITLE,
    ATTACHMENT,
    NB,
}

pub struct AvPacket {
    ptr: Pointer,
}

impl AvPacket {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { generated::av_packet() },
        }
    }

    pub fn as_ptr(&self) -> Pointer {
        self.ptr
    }

    pub fn stream_index(&self) -> usize {
        unsafe { generated::av_packet_stream_index(self.ptr) as usize }
    }
}
