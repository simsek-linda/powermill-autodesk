//! Windows registry read helpers for Powermill Autodesk.

pub const APP_KEY: &str = "Software\\Powermill\\Powermill Autodesk";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Powermill Autodesk";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Powermill Autodesk") },
        RegValue { name: "Version",     value: String::from("v2.1.4") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Powermill Autodesk") },
        RegValue { name: "Publisher",       value: String::from("Powermill") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Powermill Autodesk\\uninstall.exe\"") },
    ]
}
