/*
 * Copyright (C) 2019 Romain Vimont
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod adb_monitor;
mod byte_buffer;

use crate::adb_monitor::{AdbMonitor, AdbMonitorCallback};
use std::env;
use std::process::Command;

struct AutoAdb {
    command: Vec<String>,
}

impl AdbMonitorCallback for AutoAdb {
    fn on_new_device_connected(&self, serial: &str) {
        let cmd = self
            .command
            .iter()
            .map(|value| {
                // replace any {} parameter by the actual serial
                if "{}" == value {
                    serial.to_string()
                } else {
                    value.to_string()
                }
            })
            .collect::<Vec<_>>();
        println!("Deteted device {}", serial);
        let process = Command::new(&cmd[0]).args(cmd.iter().skip(1)).spawn();
        if let Err(err) = process {
            eprintln!("Could not execute {:?}: {}", cmd, err);
        }
    }
}

fn main() {
    let command = env::args().skip(1).collect::<Vec<_>>();
    if command.is_empty() {
        eprintln!("No arguments given");
        return;
    }
    let auto_adb = AutoAdb { command };
    let mut adb_monitor = AdbMonitor::new(Box::new(auto_adb));
    adb_monitor.monitor();
}
