use tauri::{Manager, WindowBuilder, WindowUrl};

#[tauri::command]
pub async fn open_new_window(
    app_handle: tauri::AppHandle,
    url: String,
    width: f64,
    height: f64,
    transparent: bool,
    decorations: bool,
    name: String,
) -> Result<(), String> {
    let mut attempt = 0;
    let mut window_name = name.clone();

    loop {
        match WindowBuilder::new(
            &app_handle,
            window_name.clone(),
            WindowUrl::App(url.clone().into()),
        )
        .title(&window_name)
        .inner_size(width, height) // Width and height
        .resizable(true)
        .transparent(transparent)
        .decorations(decorations)
        .build()
        {
            Ok(_) => return Ok(()),
            Err(e) => match e {
                tauri::Error::WindowLabelAlreadyExists(_) => {
                    if window_name == "scoreboard" {
                        return Err("한번에 하나만 열 수 있습니다".to_string());
                    }
                    attempt += 1;
                    window_name = format!("{}{}", name, attempt);
                }
                _ => return Err(e.to_string()),
            },
        }
    }
}

#[tauri::command]
pub async fn adjust_window_size(
    window: tauri::Window,
    width: f64,
    height: f64,
) -> Result<(), String> {
    window
        .set_size(tauri::Size::Logical(tauri::LogicalSize { width, height }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn close_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    let scoreboard = app_handle.get_window("scoreboard");
    if let Some(window) = scoreboard {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
