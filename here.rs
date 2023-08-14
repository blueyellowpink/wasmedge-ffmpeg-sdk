
pub type Pointer = u64;
#[link(wasm_import_module = "wasmedge_ffmpeg")]
extern "C" {    
                #[link_name = "wasmedge_ffmpeg_add"] 
                pub fn add(a: u32, b: u32) -> u32;
                #[link_name = "wasmedge_ffmpeg_avformat_alloc_context"] 
                pub fn avformat_alloc_context() -> Pointer;
                #[link_name = "wasmedge_ffmpeg_avformat_get_context"] 
                pub fn avformat_get_context(context: Pointer) -> u64;
                #[link_name = "wasmedge_ffmpeg_avformat_open_input"] 
                pub fn avformat_open_input(context: Pointer, filename_ptr: *const u8, filename_len: usize) -> u32;
                #[link_name = "wasmedge_ffmpeg_subtract"] 
                pub fn subtract(a: u32, b: u32) -> u32;}