use crate::command::{OptionCandidate, RegistryEditOption};
use crate::win;

struct Builder {
    id_counter: u64,
    buffer: Vec<RegistryEditOption>,
}

impl Builder {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            id_counter: 0,
            buffer: Vec::with_capacity(cap),
        }
    }
    pub fn push(
        &mut self,
        label: impl Into<String>,
        def: win::reg::RegDef,
        values: Vec<OptionCandidate>,
    ) {
        if self.id_counter < u64::MAX {
            self.id_counter += 1;
        } else {
            panic!();
        }

        self.buffer.push(RegistryEditOption {
            id: self.id_counter,
            label: label.into(),
            def,
            values,
        });
    }

    pub fn into_vec(self) -> Vec<RegistryEditOption> {
        self.buffer
    }
}

fn get_sid() -> String {
    let user = win::get_username().expect("Failed to get username");
    win::ps::get_sid(&user).expect("Failed to get SID")
}

pub fn default_edit_options() -> Vec<RegistryEditOption> {
    use win::reg::DataType;
    use win::reg::RegDef as R;
    use OptionCandidate as O;

    let sid = get_sid();

    let mut b = Builder::with_capacity(8);
    b.push(
        "エクスプローラ - 右クリックメニュー",
        R::hkcu(
            r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32",
            "",
            DataType::String,
        ),
        vec![
            O::new("", "従来のメニュー"),
            O::new("", "TODO: Windows11 のメニュー"),
        ],
    );
    b.push(
        "エクスプローラ - 起動画面",
        R::hku(
            format!(
                "{}\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
                sid.as_str()
            ),
            "LaunchTo",
            DataType::DWord,
        ),
        vec![O::new("1", "PC"), O::new("2", "ホーム")],
    );

    b.push(
        "エクスプローラ - ファイル拡張子",
        R::hku(
            format!(
                "{}\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
                sid.as_str()
            ),
            "HideFileExt",
            DataType::DWord,
        ),
        vec![
            O::new("0", "登録された拡張子を表示する"),
            O::new("1", "登録された拡張子を表示しない"),
        ],
    );

    b.push(
        "タスクバー - スタートメニュー位置",
        R::hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "TaskbarAl",
            DataType::DWord,
        ),
        vec![O::new("0", "左揃え"), O::new("1", "中央揃え")],
    );

    b.push(
        "タスクバー - 検索ボックス",
        R::hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
            "SearchBoxTaskbarMode",
            DataType::DWord,
        ),
        vec![
            O::new("0", "非表示"),
            O::new("1", "検索アイコンのみ"),
            O::new("2", "検索ボックス"),
            O::new("3", "検索アイコンとラベル"),
        ],
    );

    b.push(
        "タスクバー - タスクビュー",
        R::hkcu(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowTaskViewButton",
            DataType::DWord,
        ),
        vec![O::new("0", "非表示"), O::new("1", "表示")],
    );
    // HKU:\<SID>\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced -Name 'TaskbarMn' -Type 'DWord'
    b.push(
        "タスクバー - チャット",
        R::hku(
            format!(
                "{}\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
                sid.as_str()
            ),
            "TaskbarMn",
            DataType::DWord,
        ),
        vec![O::new("0", "非表示"), O::new("1", "表示")],
    );

    // 0x80070005: Access denied. why?
    // b.push(
    //     "タスクバー - ウィジェット非表示",
    //     R::hkcu(
    //         r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
    //         "TaskbarDa",
    //         DataType::DWord,
    //     ),
    //     vec![O::new("0", "非表示"), O::new("1", "表示")],
    // );

    b.into_vec()
}
