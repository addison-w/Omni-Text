use rdev::{simulate, EventType, Key};
use std::thread;
use std::time::Duration;

/// Simulate Cmd+C to copy selected text to clipboard
pub fn simulate_copy() -> Result<(), String> {
    let delay = Duration::from_millis(20);

    simulate(&EventType::KeyPress(Key::MetaLeft))
        .map_err(|e| format!("Failed to press Meta: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyPress(Key::KeyC))
        .map_err(|e| format!("Failed to press C: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyRelease(Key::KeyC))
        .map_err(|e| format!("Failed to release C: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyRelease(Key::MetaLeft))
        .map_err(|e| format!("Failed to release Meta: {:?}", e))?;

    // Wait for clipboard to be populated
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

/// Simulate Cmd+V to paste from clipboard
pub fn simulate_paste() -> Result<(), String> {
    let delay = Duration::from_millis(20);

    simulate(&EventType::KeyPress(Key::MetaLeft))
        .map_err(|e| format!("Failed to press Meta: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyPress(Key::KeyV))
        .map_err(|e| format!("Failed to press V: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyRelease(Key::KeyV))
        .map_err(|e| format!("Failed to release V: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyRelease(Key::MetaLeft))
        .map_err(|e| format!("Failed to release Meta: {:?}", e))?;

    // Wait for paste to complete
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

/// Simulate Cmd+A to select all (useful for replacing in single-line fields)
#[allow(dead_code)]
pub fn simulate_select_all() -> Result<(), String> {
    let delay = Duration::from_millis(20);

    simulate(&EventType::KeyPress(Key::MetaLeft))
        .map_err(|e| format!("Failed to press Meta: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyPress(Key::KeyA))
        .map_err(|e| format!("Failed to press A: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyRelease(Key::KeyA))
        .map_err(|e| format!("Failed to release A: {:?}", e))?;
    thread::sleep(delay);
    simulate(&EventType::KeyRelease(Key::MetaLeft))
        .map_err(|e| format!("Failed to release Meta: {:?}", e))?;

    thread::sleep(Duration::from_millis(50));
    Ok(())
}
