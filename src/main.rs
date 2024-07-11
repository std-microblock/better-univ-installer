use std::path::PathBuf;

use native_dialog::FileDialog;
use slint::SharedString;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

slint::include_modules!();

fn get_qqnt_path() -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\QQ").ok()?;
    let path: String  = cur_ver.get_value("DisplayIcon").ok()?;
    Some(path.split(",").next()?.to_string())
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.set_qq_path(
        get_qqnt_path().unwrap_or("".to_string()).into()
    );

    let uiw = ui.as_weak();
    ui.on_select_qqnt_path(move || {
        let path = FileDialog::new()
            .add_filter("QQNT Executable", &["QQ.exe"])
            .show_open_single_file();
        if let Ok(Some(path)) = path {
            uiw.upgrade().unwrap().set_qq_path(
                path.to_str().unwrap().to_string().into());
        }
    });

    let uiw = ui.as_weak();
    ui.on_install_bqqnt(move || {
        let ui = uiw.upgrade().unwrap();
        let qqpath = ui.get_qq_path();
        let qqpath = PathBuf::from(qqpath.to_string());
        let _ = std::fs::write(qqpath.parent().unwrap().join("version.dll"), include_bytes!("../../BetterQQNT/bin/better-qqnt-x64.dll"));
        ui.set_installed(true);
    });

    ui.run()
}
