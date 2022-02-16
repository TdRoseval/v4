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

use serde::Deserialize;
use std::path::Path;

use crate::steps::{compiler::Compiler, options::Options};

mod extension;
mod test;

#[derive(Deserialize)]
pub struct TaskArgs {
  pub vscode: String,
  pub compiler: Compiler,
  pub workspace: String,
  pub options: Options,
}

struct Task {
  name: &'static str,
  action: fn(&TaskArgs) -> Result<(), &'static str>,
  validator: fn(&TaskArgs) -> bool,
}

fn llvm_setup(setup: &str) -> bool {
  ["llvm-mingw", "llvm", "apple"].contains(&setup)
}

fn should_test(args: &TaskArgs) -> bool {
  let test = args.options.test;
  if test.is_none() {
    let hello_word_filename = if args.options.language == "C" {
      "helloworld.c"
    } else {
      "helloworld.cpp"
    };
    let hello_world_path = Path::new(&args.workspace).join(hello_word_filename);
    !hello_world_path.exists()
  } else {
    test.unwrap()
  }
}

macro_rules! generate_task {
  ($( ($t:path, $vp:pat => $vb:expr) ),* ) => {
    vec![
      $( Task {
        name: stringify!($t),
        action: $t,
        validator: (|$vp: &TaskArgs| $vb)
      } ),*
    ]
  };
}

// TODO LIST:
mod run {
  use super::TaskArgs;
  pub fn create_pauser(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
  pub fn create_keybinding(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
}
mod debug {
  use super::TaskArgs;
  pub fn create_checker(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
}
mod compiler {
  use super::TaskArgs;
  pub fn add_to_path(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
}
mod dotvscode {
  use super::TaskArgs;
  pub fn tasks_json(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
  pub fn launch_json(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
  pub fn c_cpp_properties_json(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
}
mod shortcut {
  use super::TaskArgs;
  pub fn create(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
}
mod vscode {
  use super::TaskArgs;
  pub fn open(args: &TaskArgs) -> Result<(), &'static str> {
    Err("")
  }
}

pub fn list(args: &TaskArgs) -> Vec<(&'static str, fn(&TaskArgs) -> Result<(), &'static str>)> {
  generate_task![
    (extension::remove_unrecommended, a => a.options.remove_extensions),
    (extension::install_c_cpp, _ => true),
    (extension::install_code_lldb, a => llvm_setup(&a.compiler.setup)),
    (run::create_pauser, a => !a.options.compatible_mode),
    (run::create_keybinding, a => !a.options.compatible_mode),
    (debug::create_checker, a => a.options.ascii_check),
    (compiler::add_to_path, a => a.options.add_to_path),
    (dotvscode::tasks_json, _ => true),
    (dotvscode::launch_json, _ => true),
    (dotvscode::c_cpp_properties_json, _ => true),
    (test::generate, a => should_test(&a)),
    (shortcut::create, a => a.options.desktop_shortcut),
    (vscode::open, a => a.options.open_vscode)
  ]
  .iter()
  .filter(|t| (t.validator)(&args))
  .map(|t| (t.name, t.action))
  .collect()
}
