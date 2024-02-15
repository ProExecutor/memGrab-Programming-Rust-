// Rust doesn't have a direct equivalent of #pragma once, but it employs module system for inclusion control.

extern crate glfw; // Assuming GLFW is added as a dependency

use glfw::{Context, Monitor};

pub fn get_current_monitor(window: &glfw::Window) -> Option<Monitor> {
    // Rust's `Option` type handles potential null pointers gracefully.
    unsafe {
        let monitor_ptr = glfw::GetWindowMonitor(window);
        if monitor_ptr.is_null() {
            None
        } else {
            Some(Monitor::from_ptr(monitor_ptr))
        }
    }
}
