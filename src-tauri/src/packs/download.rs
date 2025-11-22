use tauri::Emitter;

fn emit_progress(app: &tauri::AppHandle, pack_id: u64, downloaded: u64, total: u64, stage: &str) {
    let event_name = format!("download-progress-{}", pack_id);
    let _ = app.emit(
        &event_name,
        serde_json::json!({
            "packId": pack_id,
            "downloaded": downloaded,
            "total": total,
            "stage": stage
        }),
    );
}

#[tauri::command]
pub async fn download_pack(
    app: tauri::AppHandle,
    download_url: String,
    pack_id: u64,
) -> Result<String, String> {
    println!("[download_pack] Starting download from: {}", download_url);
    
    // Emit initial progress
    let _ = app.emit(
        &format!("download-progress-{}", pack_id),
        serde_json::json!({
            "packId": pack_id,
            "downloaded": 0,
            "total": 0,
            "stage": "downloading"
        }),
    );
    
    // Download the ZIP file
    let zip_path = download_file(&app, &download_url, pack_id).await?;
    
    // Emit extracting stage
    emit_progress(&app, pack_id, 100, 100, "extracting");
    
    // Extract the ZIP file
    let extract_path = extract_zip(&zip_path)?;
    
    // Emit converting stage
    emit_progress(&app, pack_id, 100, 100, "converting");
    
    // Process all .sm files found in the extracted directory
    process_sm_files(&extract_path)?;
    
    Ok(zip_path.to_string_lossy().to_string())
}

/// Downloads a file from the given URL and saves it to the downloads directory
async fn download_file(
    app: &tauri::AppHandle,
    download_url: &str,
    pack_id: u64,
) -> Result<std::path::PathBuf, String> {
    println!("[download_file] Sending HTTP request...");
    
    let client = reqwest::Client::new();
    let mut response = client
        .get(download_url)
        .header("Accept", "*/*")
        .header("Origin", "https://etternaonline.com")
        .send()
        .await
        .map_err(|e| {
            println!("[download_file] Connection error: {}", e);
            format!("Connection error: {}", e)
        })?;
    
    let status = response.status();
    println!("[download_file] Response status: {}", status);
    
    if !status.is_success() {
        let error_msg = format!("HTTP error: {}", status);
        println!("[download_file] {}", error_msg);
        return Err(error_msg);
    }
    
    // Get content length if available
    let total_size = response.content_length().unwrap_or(0);
    
    // Extract filename from URL
    let filename = download_url
        .split('/')
        .last()
        .unwrap_or("pack.zip")
        .split('?')
        .next()
        .unwrap_or("pack.zip");
    
    // Get downloads directory
    let mut download_path = std::env::current_dir()
        .map_err(|e| {
            println!("[download_file] Error getting current directory: {}", e);
            format!("Error getting current directory: {}", e)
        })?;
    download_path.push("downloads");
    
    // Create downloads directory if it doesn't exist
    std::fs::create_dir_all(&download_path)
        .map_err(|e| {
            println!("[download_file] Error creating downloads directory: {}", e);
            format!("Error creating downloads directory: {}", e)
        })?;
    
    download_path.push(filename);
    
    let file_path_str = download_path.to_string_lossy().to_string();
    println!("[download_file] Saving to: {}", file_path_str);
    
    // Create file for writing
    let mut file = tokio::fs::File::create(&download_path)
        .await
        .map_err(|e| {
            println!("[download_file] Error creating file: {}", e);
            format!("Error creating file: {}", e)
        })?;
    
    println!("[download_file] Starting streaming download...");
    
    use tokio::io::AsyncWriteExt;
    
    let mut total_bytes = 0u64;
    let mut last_emit_time = std::time::Instant::now();
    
    // Stream chunks from response to file
    loop {
        match response.chunk().await {
            Ok(Some(chunk)) => {
                file.write_all(&chunk).await.map_err(|e| {
                    println!("[download_file] Error writing chunk: {}", e);
                    format!("Error writing chunk: {}", e)
                })?;
                
                total_bytes += chunk.len() as u64;
                
                // Emit progress every 100KB or every 500ms
                if total_bytes % 102_400 == 0 || last_emit_time.elapsed().as_millis() >= 500 {
                    emit_progress(app, pack_id, total_bytes, total_size, "downloading");
                    last_emit_time = std::time::Instant::now();
                }
            }
            Ok(None) => {
                // End of stream
                break;
            }
            Err(e) => {
                println!("[download_file] Error reading chunk: {}", e);
                return Err(format!("Error reading chunk: {}", e));
            }
        }
    }
    
    // Emit final progress
    emit_progress(app, pack_id, total_bytes, total_bytes, "downloading");
    
    file.sync_all().await.map_err(|e| {
        println!("[download_file] Error syncing file: {}", e);
        format!("Error syncing file: {}", e)
    })?;
    
    let final_mb = total_bytes as f64 / 1_048_576.0;
    println!("[download_file] File saved successfully: {:.2} MB total", final_mb);
    
    Ok(download_path)
}

/// Extracts a ZIP file to a directory
fn extract_zip(zip_path: &std::path::Path) -> Result<std::path::PathBuf, String> {
    println!("[extract_zip] Extracting zip file...");
    
    let extract_path = zip_path.parent().unwrap().join(
        zip_path.file_stem().unwrap().to_string_lossy().to_string()
    );
    
    std::fs::create_dir_all(&extract_path)
        .map_err(|e| {
            println!("[extract_zip] Error creating extract directory: {}", e);
            format!("Error creating extract directory: {}", e)
        })?;
    
    let file = std::fs::File::open(zip_path)
        .map_err(|e| {
            println!("[extract_zip] Error opening zip file: {}", e);
            format!("Error opening zip file: {}", e)
        })?;
    
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| {
            println!("[extract_zip] Error reading zip archive: {}", e);
            format!("Error reading zip archive: {}", e)
        })?;
    
    archive.extract(&extract_path)
        .map_err(|e| {
            println!("[extract_zip] Error extracting zip: {}", e);
            format!("Error extracting zip: {}", e)
        })?;
    
    println!("[extract_zip] Zip extracted to: {}", extract_path.to_string_lossy());
    
    Ok(extract_path)
}

/// Processes all .sm files found in the given directory
fn process_sm_files(extract_path: &std::path::Path) -> Result<(), String> {
    println!("[process_sm_files] Searching for .sm files...");
    
    let sm_files = find_sm_files(extract_path)?;
    println!("[process_sm_files] Found {} .sm files", sm_files.len());
    
    // Collect all unique directories containing .sm files
    let mut song_dirs = std::collections::HashSet::new();
    for sm_file in &sm_files {
        if let Some(parent) = sm_file.parent() {
            song_dirs.insert(parent.to_path_buf());
        }
    }
    
    // Convert all .sm files
    for sm_file in &sm_files {
        println!("[process_sm_files] Processing .sm file: {}", sm_file.to_string_lossy());
        convert_and_save_sm_file(sm_file);
    }
    
    // Copy song directories to song_path if configured
    let settings = crate::settings::Settings::load().unwrap_or_default();
    if !settings.song_path.is_empty() {
        copy_song_directories(&song_dirs, &settings.song_path)?;
    }
    
    Ok(())
}

/// Copies song directories to the configured song path
fn copy_song_directories(
    song_dirs: &std::collections::HashSet<std::path::PathBuf>,
    song_path: &str,
) -> Result<(), String> {
    let target_path = std::path::Path::new(song_path);
    
    // Create target directory if it doesn't exist
    std::fs::create_dir_all(target_path)
        .map_err(|e| format!("Error creating song path directory: {}", e))?;
    
    println!("[copy_song_directories] Copying {} song directories to: {}", song_dirs.len(), song_path);
    
    for song_dir in song_dirs {
        let dir_name = song_dir.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| "Invalid directory name".to_string())?;
        
        let target_dir = target_path.join(dir_name);
        
        // Remove target directory if it exists
        if target_dir.exists() {
            std::fs::remove_dir_all(&target_dir)
                .map_err(|e| format!("Error removing existing directory: {}", e))?;
        }
        
        // Copy directory
        copy_dir_all(song_dir, &target_dir)
            .map_err(|e| format!("Error copying directory {}: {}", song_dir.display(), e))?;
        
        println!("[copy_song_directories] Copied {} to {}", song_dir.display(), target_dir.display());
    }
    
    Ok(())
}

/// Recursively copies a directory
fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dst)
        .map_err(|e| format!("Error creating destination directory: {}", e))?;
    
    for entry in std::fs::read_dir(src)
        .map_err(|e| format!("Error reading source directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);
        
        if path.is_dir() {
            copy_dir_all(&path, &dst_path)?;
        } else {
            std::fs::copy(&path, &dst_path)
                .map_err(|e| format!("Error copying file: {}", e))?;
        }
    }
    
    Ok(())
}

/// Converts a single .sm file to .osu format and saves the results
fn convert_and_save_sm_file(sm_file: &std::path::Path) {
    // Read file content
    let file_content = match std::fs::read(sm_file) {
        Ok(content) => content,
        Err(e) => {
            println!("[convert_and_save_sm_file] Error reading .sm file {}: {}", sm_file.display(), e);
            return;
        }
    };
    
    // Convert .sm to .osu
    let osu_files = match crate::maps::from_sm_to_osu(file_content) {
        Ok(files) => files,
        Err(e) => {
            println!("[convert_and_save_sm_file] Error converting .sm to .osu: {}", e);
            return;
        }
    };
    
    println!("[convert_and_save_sm_file] Successfully converted to {} .osu file(s)", osu_files.len());
    
    // Get the base name and parent directory of the .sm file
    let Some(sm_dir) = sm_file.parent() else {
        println!("[convert_and_save_sm_file] Could not get parent directory of {}", sm_file.display());
        return;
    };
    
    let base_name = sm_file.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    // Save each .osu file next to the .sm file
    for (difficulty, osu_bytes) in osu_files {
        let osu_filename = format!("{} - {}.osu", base_name, difficulty);
        let osu_path = sm_dir.join(&osu_filename);
        
        match std::fs::write(&osu_path, &osu_bytes) {
            Ok(_) => {
                println!("[convert_and_save_sm_file] Saved .osu file: {}", osu_path.display());
            }
            Err(e) => {
                println!("[convert_and_save_sm_file] Error saving .osu file {}: {}", osu_path.display(), e);
            }
        }
    }
}

fn find_sm_files(dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut sm_files = Vec::new();
    
    fn walk_dir(dir: &std::path::Path, sm_files: &mut Vec<std::path::PathBuf>) -> Result<(), String> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Error reading directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                walk_dir(&path, sm_files)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("sm") {
                sm_files.push(path);
            }
        }
        
        Ok(())
    }
    
    walk_dir(dir, &mut sm_files)?;
    Ok(sm_files)
}

