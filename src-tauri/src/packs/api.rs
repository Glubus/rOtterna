use super::types::PacksResponse;
use super::utils::SortField;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SortOption {
    pub value: String,
    pub label: String,
}

#[tauri::command]
pub fn get_sort_options() -> Vec<SortOption> {
    vec![
        SortOption {
            value: "name".to_string(),
            label: "Name".to_string(),
        },
        SortOption {
            value: "popularity".to_string(),
            label: "Popularity".to_string(),
        },
        SortOption {
            value: "overall".to_string(),
            label: "Overall".to_string(),
        },
        SortOption {
            value: "stream".to_string(),
            label: "Stream".to_string(),
        },
        SortOption {
            value: "jumpstream".to_string(),
            label: "Jumpstream".to_string(),
        },
        SortOption {
            value: "handstream".to_string(),
            label: "Handstream".to_string(),
        },
        SortOption {
            value: "jacks".to_string(),
            label: "Jacks".to_string(),
        },
        SortOption {
            value: "chordjacks".to_string(),
            label: "Chordjacks".to_string(),
        },
        SortOption {
            value: "stamina".to_string(),
            label: "Stamina".to_string(),
        },
        SortOption {
            value: "technical".to_string(),
            label: "Technical".to_string(),
        },
    ]
}

#[tauri::command]
pub async fn fetch_packs(
    page: Option<u64>,
    limit: Option<u64>,
    sort: Option<String>,
    search: Option<String>,
) -> Result<PacksResponse, String> {
    println!("[fetch_packs] Starting fetch with params: page={:?}, limit={:?}, sort={:?}, search={:?}", page, limit, sort, search);
    
    let client = reqwest::Client::new();
    
    let mut url = reqwest::Url::parse("https://api.etternaonline.com/api/packs")
        .map_err(|e| {
            println!("[fetch_packs] URL parse error: {}", e);
            format!("URL parse error: {}", e)
        })?;
    
    println!("[fetch_packs] Base URL parsed successfully");
    
    if let Some(p) = page {
        url.query_pairs_mut().append_pair("page", &p.to_string());
        println!("[fetch_packs] Added page parameter: {}", p);
    }
    if let Some(l) = limit {
        url.query_pairs_mut().append_pair("limit", &l.to_string());
        println!("[fetch_packs] Added limit parameter: {}", l);
    }
    if let Some(s) = sort {
        println!("[fetch_packs] Validating sort parameter: {}", s);
        let sort_str = s.trim();
        let is_descending = sort_str.starts_with('-');
        let field_str = if is_descending {
            &sort_str[1..]
        } else {
            sort_str
        };
        
        if SortField::from_str(field_str).is_none() {
            let error_msg = format!(
                "Invalid sort field: {}. Valid options: name, popularity, overall, stream, jumpstream, handstream, jacks, chordjacks, stamina, technical",
                field_str
            );
            println!("[fetch_packs] {}", error_msg);
            return Err(error_msg);
        }
        
        url.query_pairs_mut().append_pair("sort", &s);
        println!("[fetch_packs] Added sort parameter: {}", s);
    }
    if let Some(search_term) = search {
        if !search_term.trim().is_empty() {
            url.query_pairs_mut().append_pair("filter[search]", &search_term.trim());
            println!("[fetch_packs] Added search parameter: {}", search_term.trim());
        }
    }
    
    let final_url = url.as_str();
    println!("[fetch_packs] Final URL: {}", final_url);
    
    println!("[fetch_packs] Sending HTTP request...");
    let response = client
        .get(final_url)
        .header("Accept", "application/json, text/plain, */*")
        .header("Origin", "https://etternaonline.com")
        .send()
        .await
        .map_err(|e| {
            println!("[fetch_packs] Connection error: {}", e);
            format!("Connection error: {}", e)
        })?;
    
    let status = response.status();
    println!("[fetch_packs] Response status: {}", status);
    
    if !status.is_success() {
        let error_msg = format!("HTTP error: {}", status);
        println!("[fetch_packs] {}", error_msg);
        return Err(error_msg);
    }
    
    println!("[fetch_packs] Reading response body...");
    let response_text = response
        .text()
        .await
        .map_err(|e| {
            println!("[fetch_packs] Error reading response body: {}", e);
            format!("Error reading response body: {}", e)
        })?;
    
    println!("[fetch_packs] Response body length: {} bytes", response_text.len());
    println!("[fetch_packs] Response body preview (first 500 chars): {}", 
        if response_text.len() > 500 {
            &response_text[..500]
        } else {
            &response_text
        });
    
    println!("[fetch_packs] Parsing JSON response...");
    let packs_response: PacksResponse = serde_json::from_str(&response_text)
        .map_err(|e| {
            println!("[fetch_packs] JSON parse error: {}", e);
            println!("[fetch_packs] Error at position: {:?}", e);
            format!("JSON parse error: {}", e)
        })?;
    
    println!("[fetch_packs] Successfully parsed {} packs", packs_response.data.len());
    Ok(packs_response)
}

