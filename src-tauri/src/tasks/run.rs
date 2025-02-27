// Copyright (C) 2022 Guyutongxue
//
// This file is part of vscch4.
//
// vscch4 is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// vscch4 is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with vscch4.  If not, see <http://www.gnu.org/licenses/>.

use std::fs;
use std::io::BufReader;
#[cfg(not(target_os = "windows"))]
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use log::{debug, trace, warn};
use serde_json::json;

use super::TaskArgs;

#[cfg(target_os = "windows")]
mod scripts {
  pub static PAUSE_CONSOLE_SCRIPT_NAME: &str = "pause-console.ps1";
  pub static PAUSE_CONSOLE_SCRIPT: &str = include_str!("../scripts/pause-console.ps1");

  pub static CHECK_ASCII_SCRIPT_NAME: &str = "check-ascii.ps1";
  pub static CHECK_ASCII_SCRIPT: &str = include_str!("../scripts/check-ascii.ps1");
}

#[cfg(target_os = "macos")]
mod scripts {
  pub static PAUSE_CONSOLE_SCRIPT_NAME: &str = "pause-console.rb";
  pub static PAUSE_CONSOLE_SCRIPT: &str = include_str!("../scripts/pause-console.rb");

  pub static PAUSE_CONSOLE_LAUNCHER_SCRIPT_NAME: &str = "pause-console-launcher.sh";
  pub static PAUSE_CONSOLE_LAUNCHER_SCRIPT: &str =
    include_str!("../scripts/pause-console-launcher.sh");
}

#[cfg(target_os = "linux")]
mod scripts {
  pub static PAUSE_CONSOLE_SCRIPT_NAME: &str = "pause-console.sh";
  pub static PAUSE_CONSOLE_SCRIPT: &str = include_str!("../scripts/pause-console.sh");
}
pub use scripts::*;

pub fn script_path() -> Option<PathBuf> {
  dirs::data_dir().map(|p| p.join("vscch"))
}

fn save_script(filename: &str, content: &str) -> Result<()> {
  let path = script_path().ok_or(anyhow!("找不到用于存放脚本的路径。"))?;
  fs::create_dir_all(&path)?;
  let filepath = path.join(filename);
  fs::write(&filepath, content)?;
  #[cfg(not(target_os = "windows"))]
  {
    fs::set_permissions(&filepath, fs::Permissions::from_mode(0o755))?;
  }
  Ok(())
}

pub fn create_pauser(_: &TaskArgs) -> Result<()> {
  #[cfg(target_os = "macos")]
  save_script(
    PAUSE_CONSOLE_LAUNCHER_SCRIPT_NAME,
    PAUSE_CONSOLE_LAUNCHER_SCRIPT,
  )?;
  save_script(PAUSE_CONSOLE_SCRIPT_NAME, PAUSE_CONSOLE_SCRIPT)?;

  #[cfg(target_os = "macos")]
  {
    set_terminal_preferences()?;
  }

  Ok(())
}

#[cfg(target_os = "windows")]
pub fn create_checker(_: &TaskArgs) -> Result<()> {
  save_script(CHECK_ASCII_SCRIPT_NAME, CHECK_ASCII_SCRIPT)
}

#[cfg(not(target_os = "windows"))]
pub fn create_checker(_: &TaskArgs) -> Result<()> {
  panic!("Not supported on this platform")
}

pub fn create_keybinding(_: &TaskArgs) -> Result<()> {
  let key = "f6";
  let command = "workbench.action.tasks.runTask";
  let args = "run and pause";

  let filepath = dirs::config_dir()
    .ok_or(anyhow!("找不到配置文件存放的路径。"))?
    .join("Code")
    .join("User")
    .join("keybindings.json");
  fs::create_dir_all(filepath.parent().unwrap())?;
  let mut result = vec![];
  if filepath.exists() {
    let file = fs::File::open(&filepath)?;
    let reader = BufReader::new(file);
    let content: Vec<serde_json::Value> = serde_json::from_reader(reader)?;
    for i in &content {
      let this_key = i["key"].as_str().ok_or(anyhow!(
        "keybindings.json 中的 \"key\" 字段应为 string 类型。"
      ))?;
      if this_key == key {
        // Warning for overwriting
        warn!("快捷键 {} 已有配置，将被覆盖。", key);
      } else {
        result.push(i.clone());
      }
    }
  }
  result.push(json!({
    "key": key,
    "command": command,
    "args": args
  }));
  let result = serde_json::Value::Array(result);
  fs::write(filepath, serde_json::to_string_pretty(&result)?)?;
  Ok(())
}

#[cfg(target_os = "macos")]
fn set_terminal_preferences() -> Result<()> {
  debug!("写入设置到 com.apple.Terminal.plist 中...");
  let preference_path = dirs::home_dir()
    .ok_or(anyhow!("找不到家目录。"))?
    .join("Library/Preferences/com.apple.Terminal.plist");
  let mut preference = plist::Value::from_file(&preference_path)?;
  trace!("{:?}", preference);
  let preference = preference
    .as_dictionary_mut()
    .ok_or(anyhow!(".plist 不是一个字典。"))?;
  let default_profile_name = preference
    .get("Default Window Settings")
    .and_then(|v| v.as_string())
    .ok_or(anyhow!("Default Window Settings 字段解析错误。"))?
    .to_owned();
  let default_profile = preference
    .get_mut("Window Settings")
    .and_then(|v| v.as_dictionary_mut())
    .and_then(|v| v.get_mut(&default_profile_name))
    .and_then(|v| v.as_dictionary_mut())
    .ok_or(anyhow!("Window Settings 字段解析错误。"))?;
  default_profile.insert("shellExitAction".to_string(), plist::Value::from(0));
  plist::to_file_binary(preference_path, preference)?;
  debug!("写入设置完成。");
  Ok(())
}
