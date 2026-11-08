//! Windows registry read helpers for Unreal Editor Fortnite.

pub const APP_KEY: &str = "Software\\Unreal\\Unreal Editor Fortnite";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Unreal Editor Fortnite";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Unreal Editor Fortnite") },
        RegValue { name: "Version",     value: String::from("v4.2.3") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Unreal Editor Fortnite") },
        RegValue { name: "Publisher",       value: String::from("Unreal") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Unreal Editor Fortnite\\uninstall.exe\"") },
    ]
}
