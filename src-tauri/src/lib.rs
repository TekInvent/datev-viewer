use datev_core::models::DatevFile;
use datev_core::process_decoded_file;
use datev_core::reader::read_file;
use serde::{Deserialize, Serialize};
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenFileError {
    pub message: String,
    pub encoding: Option<String>,
    pub line_ending: Option<String>,
    pub offending_line: Option<String>,
    pub line_number: Option<usize>,
}

#[tauri::command]
fn open_datev_file(path: String) -> Result<DatevFile, OpenFileError> {
    let decoded = match read_file(&path) {
        Ok(d) => d,
        Err(e) => {
            return Err(OpenFileError {
                message: format!("Failed to read file: {e}"),
                encoding: None,
                line_ending: None,
                offending_line: None,
                line_number: None,
            });
        }
    };

    match process_decoded_file(&decoded) {
        Ok(datev_file) => Ok(datev_file),
        Err(e) => Err(OpenFileError {
            message: e.message,
            encoding: Some(decoded.encoding.to_string()),
            line_ending: Some(decoded.line_ending.to_string()),
            offending_line: e.offending_line,
            line_number: e.line_number,
        }),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle();

            let open_item = MenuItemBuilder::new("Open DATEV File\u{2026}")
                .id("open_file")
                .accelerator("CmdOrCtrl+O")
                .build(app)?;

            let close_item = MenuItemBuilder::new("Close File")
                .id("close_file")
                .accelerator("CmdOrCtrl+W")
                .build(app)?;

            let reload_item = MenuItemBuilder::new("Reload File")
                .id("reload_file")
                .accelerator("CmdOrCtrl+R")
                .build(app)?;

            let quit_item = PredefinedMenuItem::quit(app_handle, Some("Quit"))?;

            let file_menu = SubmenuBuilder::new(app, "File")
                .item(&open_item)
                .item(&close_item)
                .item(&reload_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let about_item = PredefinedMenuItem::about(
                app_handle,
                Some("About Teki DATEV Viewer"),
                Some(tauri::menu::AboutMetadata {
                    name: Some("Teki DATEV Viewer".into()),
                    version: Some(app.package_info().version.to_string()),
                    authors: Some(vec!["Teki".into()]),
                    comments: Some("Read-only DATEV inspection tool".into()),
                    ..Default::default()
                }),
            )?;

            let help_menu = SubmenuBuilder::new(app, "Help").item(&about_item).build()?;

            #[cfg(target_os = "macos")]
            let app_menu = SubmenuBuilder::new(app, "Teki DATEV Viewer")
                .item(&about_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let menu_builder = MenuBuilder::new(app);

            #[cfg(target_os = "macos")]
            let menu_builder = menu_builder.item(&app_menu);

            let menu = menu_builder.item(&file_menu).item(&help_menu).build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle, event| match event.id().0.as_str() {
                "open_file" => {
                    let _ = app_handle.emit("menu-open-file", ());
                }
                "close_file" => {
                    let _ = app_handle.emit("menu-close-file", ());
                }
                "reload_file" => {
                    let _ = app_handle.emit("menu-reload-file", ());
                }
                _ => {}
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_datev_file, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_open_datev_file_non_existent() {
        let res = open_datev_file("non_existent_file_path_12345.csv".to_string());
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.message.contains("Failed to read file"));
        assert!(err.encoding.is_none());
    }

    #[test]
    fn test_open_datev_file_valid() {
        let temp_path = "test_temp_open_datev_file.csv";
        let csv_data = concat!(
            "\"EXTF\";700;21;\"Buchungsstapel\";13;20240130140440;;\"RE\";;;29098;55003;20240101;4;20240101;20240831;\"Buchungsstapel\";\"WD\";1;\"EUR\";;;;;;;;;\r\n",
            "\"Umsatz\";\"Soll/Haben\";\"Belegdatum\";\"Konto\";\"Gegenkonto\";\"BU-Schlüssel\";\"Belegfeld 1\";\"Buchungstext\"\r\n",
            "\"123,45\";\"S\";\"3001\";\"1200\";\"1600\";\"\";\"RE-100\";\"Test Booking 1\"\r\n",
            "\"67,89\";\"H\";\"3101\";\"1400\";\"8400\";\"3\";\"RE-101\";\"Test Booking 2\"\r\n"
        );
        fs::write(temp_path, csv_data).unwrap();

        let res = open_datev_file(temp_path.to_string());
        let _ = fs::remove_file(temp_path); // clean up

        assert!(res.is_ok());
        let datev_file = res.unwrap();
        assert_eq!(datev_file.header.format_identifier, "EXTF");
        assert_eq!(datev_file.records.len(), 2);
    }
}
