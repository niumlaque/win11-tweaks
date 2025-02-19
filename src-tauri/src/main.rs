// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::LazyLock;
use win11_tweaks_lib::command::RegistryEditOption;
use win11_tweaks_lib::win;

static EDIT_OPT_LIST: LazyLock<Vec<RegistryEditOption>> =
    LazyLock::new(win11_tweaks_lib::default_edit_options);

static EDIT_OPT_MAP: LazyLock<HashMap<u64, &RegistryEditOption>> = LazyLock::new(|| {
    EDIT_OPT_LIST.iter().fold(HashMap::new(), |mut acc, x| {
        acc.insert(x.id, x);
        acc
    })
});

#[tauri::command]
fn log(text: &str) {
    println!("{text}");
}

fn get_component_html(cmd: &RegistryEditOption) -> String {
    let items = cmd
        .values
        .iter()
        .map(|x| {
            format!(
                "<option value=\"{}\">{}: {}</option>",
                x.value, x.value, x.description
            )
        })
        .collect::<Vec<String>>()
        .join("");
    format!(
        r#"<div class="group" data-cmdid="{}">
  <div class="group-header">{}</div>
  <div class="group-body">
    <div class="input-row">
      <input type="text" class="textbox" value="{}" readonly />
      <button class="button button-check">チェック</button>
    </div>
    <div class="input-row">
      <select class="combobox">{}</select>
      <button class="button button-exec">値設定</button>
    </div>
  </div>
</div>"#,
        cmd.id, cmd.label, cmd.def, items
    )
}

#[tauri::command]
fn get_default_components() -> Vec<String> {
    EDIT_OPT_LIST
        .iter()
        .map(get_component_html)
        .collect::<Vec<String>>()
}

#[tauri::command]
fn get_registry_value(cmd_id: u64) {
    use win::reg::Registry;
    println!("get_registry_value: Command ID={cmd_id}");
    if let Some(cmd) = EDIT_OPT_MAP.get(&cmd_id) {
        let r = Registry::new(cmd.def.root(), &cmd.def.sub_key, &cmd.def.value_name);
        match r.get_value(cmd.def.data_type) {
            Ok(v) => win::message_box(format!("現在の値: {v}"), "Win11 Tweaks"),
            Err(e) => win::message_box(format!("{e}"), "Win11 Tweaks"),
        }
    } else {
        win::message_box("コマンドが見つかりませんでした", "Win11 Tweaks");
    }
}

#[tauri::command]
fn set_registry_value(cmd_id: u64, value: &str) {
    use win::reg::Registry;
    println!("set_registry_value: Command ID={cmd_id}, Value={value}");
    if let Some(cmd) = EDIT_OPT_MAP.get(&cmd_id) {
        let r = Registry::new(cmd.def.root(), &cmd.def.sub_key, &cmd.def.value_name);
        match r.set_value(cmd.def.data_type, value) {
            Ok(_) => (),
            Err(e) => win::message_box(format!("{e}"), "Win11 Tweaks"),
        }
    } else {
        win::message_box("コマンドが見つかりませんでした", "Win11 Tweaks");
    }
}

fn inner_run() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            log,
            get_default_components,
            get_registry_value,
            set_registry_value,
        ])
        .run(tauri::generate_context!())
        .map_err(|_| anyhow::anyhow!("error while running tauri application"))?;

    Ok(())
}

pub fn run() {
    match inner_run() {
        Ok(_) => (),
        Err(e) => win::message_box(e.to_string(), "Win11 Tweaks"),
    }
}

fn main() {
    run()
}
