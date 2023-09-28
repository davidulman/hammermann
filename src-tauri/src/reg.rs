

use winreg::RegKey;
use winreg::enums::*;


#[tauri::command]
fn get_usb_keys() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let usb_key = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Enum\\USB")?;
    let keys = usb_key.enum_keys()?.map(|k| k.to_string()).collect();
    Ok(keys)
}

#[tauri::command]
fn delete_key(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let usb_key = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Enum\\USB", KEY_WRITE)?;
    usb_key.delete_subkey_all(key)?;
    Ok(())
}
