// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::LazyLock;
use win11_tweaks_lib::command::{Command, CommandManager};
use win11_tweaks_lib::win;

static COMMAND_LIST: LazyLock<Vec<Command>> = LazyLock::new(|| {
    use win::reg::DataType;
    use win::reg::RegDef as R;
    use win11_tweaks_lib::command::Value as V;

    let mut cm = CommandManager::default();
    let mut cmds = Vec::with_capacity(8);
    cmds.push(cm.gen(
        "スタートメニュー位置",
        R::hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarAl",
            DataType::DWord,
        ),
        vec![V::new("0", "左揃え"), V::new("1", "中央揃え")],
    ));
    cmds.push(cm.gen(
        "タスクバー検索ボックス非表示",
        R::hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
            "SearchBoxTaskbarMode",
            DataType::DWord,
        ),
        vec![
            V::new("0", "非表示"),
            V::new("1", "検索アイコンのみ"),
            V::new("2", "検索ボックス"),
            V::new("3", "検索アイコンとラベル"),
        ],
    ));

    cmds
});

static COMMAND_MAP: LazyLock<HashMap<u64, &Command>> = LazyLock::new(|| {
    COMMAND_LIST.iter().fold(HashMap::new(), |mut acc, x| {
        acc.insert(x.id, x);
        acc
    })
});

#[tauri::command]
fn log(text: &str) {
    println!("{text}");
}

fn get_component_html(cmd: &Command) -> String {
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
    COMMAND_LIST
        .iter()
        .map(get_component_html)
        .collect::<Vec<String>>()
}

#[tauri::command]
fn get_registry_value(cmd_id: u64) {
    use win::reg::Registry;
    println!("get_registry_value: Command ID={cmd_id}");
    if let Some(cmd) = COMMAND_MAP.get(&cmd_id) {
        let r = Registry::new(cmd.def.root(), &cmd.def.sub_key, &cmd.def.value_name);
        match r.get_value(cmd.def.data_type) {
            Ok(v) => win::message_box(format!("現在の値: {v}"), "Win11 Tweaks"),
            Err(e) => win::message_box(format!("Win32 Error: {e}"), "Win11 Tweaks"),
        }
    } else {
        win::message_box("コマンドが見つかりませんでした", "Win11 Tweaks");
    }
}

#[tauri::command]
fn set_registry_value(cmd_id: u64, value: &str) {
    use win::reg::Registry;
    println!("set_registry_value: Command ID={cmd_id}, Value={value}");
    if let Some(cmd) = COMMAND_MAP.get(&cmd_id) {
        let r = Registry::new(cmd.def.root(), &cmd.def.sub_key, &cmd.def.value_name);
        match r.set_value(cmd.def.data_type, value) {
            Ok(_) => (),
            Err(e) => win::message_box(format!("Win32 Error: {e}"), "Win11 Tweaks"),
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    match inner_run() {
        Ok(_) => (),
        Err(e) => win::message_box(e.to_string(), "Win11 Tweaks"),
    }
}

fn main() {
    run()
}
