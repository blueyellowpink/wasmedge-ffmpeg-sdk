use wasmedge_ffmpeg_sdk::*;

fn main() {
    let mut stream_index = 0;
    let mut input_context = AvFormatContext::new();
    let mut output_context = AvFormatContext::new();

    let in_filename = "./small_bunny_1080p_60fps.mp4";
    let out_filename = "./small_bunny_1080p_60fps.ts";

    avformat_open_input(&mut input_context, in_filename).unwrap();
    avformat_find_stream_info(&mut input_context).unwrap();
    output_context = avformat_alloc_output_context2(&output_context, out_filename).unwrap();

    let nb_streams = input_context.nb_streams();

    let mut streams_list: Vec<i64> = vec![0; nb_streams as usize];
    for i in 0..streams_list.len() {
        let in_stream = input_context.streams(i);
        let input_codec_params = in_stream.codecpar();
        let codec_type = input_codec_params.codec_type().unwrap();
        if codec_type != AvMediaType::AUDIO
            && codec_type != AvMediaType::VIDEO
            && codec_type != AvMediaType::SUBTITLE
        {
            streams_list[i] = -1;
            continue;
        }
        streams_list[i] = stream_index;
        stream_index += 1;

        let out_stream = avformat_new_stream(&mut output_context).unwrap();
        let output_codec_params = out_stream.codecpar();
        avcodec_params_copy(&output_codec_params, &input_codec_params).unwrap();
    }

    av_dump_format(&output_context, 0, out_filename, true);
    avio_open(&output_context, out_filename).unwrap();
    avformat_write_header(&mut output_context).unwrap();

    let packet = AvPacket::new();
    // loop {

    for _ in 0..10 {
        if av_read_frame(&input_context, &packet).is_err() {
            println!("ERROR read frame");
        }
        let packet_stream_index = packet.stream_index();
        if packet_stream_index >= nb_streams as usize || streams_list[packet_stream_index] < 0 {
            av_packet_unref(&packet);
            continue;
        }
        copy_packet(
            &input_context,
            &output_context,
            &packet,
            streams_list[packet_stream_index] as usize,
        );

        av_packet_unref(&packet);
    }

    // }
}
