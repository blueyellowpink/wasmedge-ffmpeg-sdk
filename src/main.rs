use wasmedge_ffmpeg_sdk::*;

fn main() {
    let mut input_context = AvFormatContext::new();
    let mut output_context = AvFormatContext::new();

    avformat_open_input(&mut input_context, "./small_bunny_1080p_60fps.mp4").unwrap();
    avformat_find_stream_info(&mut input_context).unwrap();
    avformat_alloc_output_context2(&mut output_context, "./small_bunny_1080p_60fps.ts").unwrap();

    let nb_streams = input_context.nb_streams();
    println!("{}", nb_streams);

    // let mut streams_list: Vec<i64> = vec![0; nb_streams as usize];
    // for i in 0..streams_list.len() {
    //     let in_stream = input_context.streams(i);
    //     streams_list[i] = -1;
    // }
}
