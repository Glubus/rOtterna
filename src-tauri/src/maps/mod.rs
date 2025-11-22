use rotterna_lib::structs::{SmFile, OsuSettings};
use rotterna_lib::converter::osu::create_basic_osu;
use crate::settings::Settings;
use std::path::PathBuf;

/// Converts a .sm file buffer to .osu format
/// Returns a Vec of (difficulty_name, osu_content_bytes) tuples
pub fn from_sm_to_osu(file_buff: Vec<u8>) -> Result<Vec<(String, Vec<u8>)>, String> {
    println!("[from_sm_to_osu] Converting .sm file to .osu format...");
    println!("[from_sm_to_osu] File size: {} bytes", file_buff.len());
    
    // Write to temporary file and use SmFile::from_file
    // (rotterna-lib expects a file path)
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("temp_sm_{}.sm", std::process::id()));
    
    std::fs::write(&temp_file, &file_buff)
        .map_err(|e| format!("Error writing temp file: {}", e))?;
    
    // Parse SM file using rotterna-lib
    let sm_file = SmFile::from_file(temp_file.clone())
        .map_err(|e| format!("Error parsing SM file: {}", e))?;
    
    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);
    
    println!("[from_sm_to_osu] Parsed .sm file: {} - {} ({} charts)", 
        sm_file.metadata.title, sm_file.metadata.artist, sm_file.charts.len());
    
    // Load settings
    let settings = Settings::load().unwrap_or_default();
    
    let mut results = Vec::new();
    
    // Convert each chart
    for chart in &sm_file.charts {
        println!("[from_sm_to_osu] Converting chart: {} ({})", 
            chart.difficulty.trim_end_matches(':'), 
            chart.stepstype.trim_end_matches(':'));
        
        // Create OsuSettings from app settings
        let osu_settings = OsuSettings {
            hp: settings.hp_drain_rate,
            od: settings.overall_difficulty,
        };
        
        // Convert to .osu format
        match create_basic_osu(&sm_file, chart, &osu_settings) {
            Ok(osu_content) => {
                let difficulty_name = chart.difficulty.trim_end_matches(':').to_string();
                results.push((difficulty_name, osu_content.into_bytes()));
            }
            Err(e) => {
                println!("[from_sm_to_osu] Error converting chart {}: {}", 
                    chart.difficulty.trim_end_matches(':'), e);
            }
        }
    }
    
    println!("[from_sm_to_osu] Generated {} .osu file(s)", results.len());
    
    Ok(results)
}

