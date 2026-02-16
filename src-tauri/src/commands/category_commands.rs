use tauri::State;
use crate::state::app_state::AppState;
use crate::core::category::{Category, CategoryStats};
use std::path::PathBuf;

#[tauri::command]
pub async fn get_categories(
    state: State<'_, AppState>,
) -> Result<Vec<Category>, String> {
    state.db.get_all_categories()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_category(
    state: State<'_, AppState>,
    category_id: String,
) -> Result<Category, String> {
    state.db.get_category(&category_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_category(
    state: State<'_, AppState>,
    name: String,
    color: Option<String>,
    icon: Option<String>,
    save_path: Option<String>,
) -> Result<Category, String> {
    let save_path_buf = save_path.map(PathBuf::from);
    let category = Category::new(name, color, icon, save_path_buf);
    
    state.db.create_category(&category)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(category)
}

#[tauri::command]
pub async fn update_category(
    state: State<'_, AppState>,
    category_id: String,
    name: Option<String>,
    color: Option<String>,
    icon: Option<String>,
    save_path: Option<String>,
) -> Result<(), String> {
    let mut category = state.db.get_category(&category_id)
        .await
        .map_err(|e| e.to_string())?;
    
    if let Some(n) = name {
        category.name = n;
    }
    if let Some(c) = color {
        category.color = Some(c);
    }
    if let Some(i) = icon {
        category.icon = Some(i);
    }
    if let Some(p) = save_path {
        category.save_path = Some(PathBuf::from(p));
    }
    
    category.updated_at = chrono::Utc::now().timestamp();
    
    state.db.update_category(&category)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_category(
    state: State<'_, AppState>,
    category_id: String,
) -> Result<(), String> {
    // Don't allow deleting the default category
    if category_id == "default" {
        return Err("Cannot delete default category".to_string());
    }
    
    state.db.delete_category(&category_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_category_stats(
    state: State<'_, AppState>,
    category_id: String,
) -> Result<CategoryStats, String> {
    state.db.get_category_stats(&category_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn assign_download_category(
    state: State<'_, AppState>,
    download_id: String,
    category_id: String,
) -> Result<(), String> {
    state.db.assign_download_category(&download_id, &category_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn auto_categorize_download(
    state: State<'_, AppState>,
    download_id: String,
    file_name: String,
) -> Result<String, String> {
    // Detect category from file extension
    let extension = file_name
        .rsplit('.')
        .next()
        .unwrap_or("");
    
    let category_id = Category::detect_from_extension(extension);
    
    // Assign the category
    state.db.assign_download_category(&download_id, &category_id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(category_id)
}
