use super::Result;
use std::process::Command;

pub fn get_sid(user: &str) -> Result<String> {
    let cmd = format!(
        "Get-CimInstance -Filter \"name='{}'\" win32_useraccount | Select-Object -ExpandProperty SID",
        user
    );
    let output = Command::new("PowerShell")
        .arg("-Command")
        .arg(&cmd)
        .output()?;

    let stdout: String = String::from_utf8_lossy(&output.stdout).into();
    Ok(stdout.trim().into())
}
