#[tauri::command]
fn log(text: &str) {
    println!("{text}");
}

fn get_component_html(label: impl AsRef<str>, value: impl AsRef<str>) -> String {
    format!(
        r#"
        <div class="group">
          <h2>{}</h2>
          <div class="group-content">
            <input type="text" value="{}" readonly />
            <button>Button1</button>
            <button>Button2</button>
          </div>
        </div>
        "#,
        label.as_ref(),
        value.as_ref()
    )
}

#[tauri::command]
fn get_default_components() -> Vec<String> {
    vec![
        get_component_html("Label1", "Value1"),
        get_component_html("Label2", "Value2"),
        get_component_html("Label3", "Value3"),
        get_component_html("Label4", "Value4"),
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![log, get_default_components])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
