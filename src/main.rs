use wasmedge_ffmpeg_sdk::{add, subtract, AvFormat};

fn main() {
    let two = add(1u32, 1u32);
    println!("two: {two}");

    let zero = subtract(1u32, 1u32);
    println!("zero: {zero}");

    let context_ptr = AvFormat::alloc_context();
    println!("context ptr: {}", context_ptr);

    let res = AvFormat::get_context(context_ptr);
    println!("context res: {}", res);

    // let open = AvFormat::open_input(context, "../small_bunny_1080p_60fps.mp4");
}
