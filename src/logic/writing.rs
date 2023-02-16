use super::*;

pub fn write(checks: Vec<Check>, pak: &std::path::PathBuf) {
    let pak = unpak::Pak::new_from_path(pak.join("Blue Fire-WindowsNoEditor.pak"), None);
}
