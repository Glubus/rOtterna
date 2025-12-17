use rhythm_open_exchange::codec::formats::osu::OsuEncoder;
use rhythm_open_exchange::codec::formats::sm::SmDecoder;
use rhythm_open_exchange::codec::{Decoder, Encoder};

/// Converts a .sm file buffer to .osu format
/// Returns a Vec of (difficulty_name, osu_content_bytes) tuples
pub fn from_sm_to_osu(file_buff: Vec<u8>) -> Result<Vec<(String, Vec<u8>)>, String> {
    println!("[from_sm_to_osu] Converting .sm file to .osu format...");
    println!("[from_sm_to_osu] File size: {} bytes", file_buff.len());

    // Decode SM file using rhythm-open-exchange
    let chart =
        SmDecoder::decode(&file_buff).map_err(|e| format!("Error decoding SM file: {}", e))?;

    println!(
        "[from_sm_to_osu] Decoded .sm file: {} - {} ({}K, {} notes)",
        chart.metadata.title,
        chart.metadata.artist,
        chart.key_count,
        chart.notes.len()
    );

    // Encode to osu! format
    let osu_data =
        OsuEncoder::encode(&chart).map_err(|e| format!("Error encoding to osu!: {}", e))?;

    // Use the chart's difficulty name or default to "Unknown"
    let difficulty_name = if chart.metadata.difficulty_name.is_empty() {
        "Unknown".to_string()
    } else {
        chart.metadata.difficulty_name.clone()
    };

    println!(
        "[from_sm_to_osu] Generated 1 .osu file (difficulty: {})",
        difficulty_name
    );

    Ok(vec![(difficulty_name, osu_data)])
}
