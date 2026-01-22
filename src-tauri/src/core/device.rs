use rdev::{Event, EventType, listen};
use serde::Serialize;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Runtime, command};

#[derive(Debug, Clone, Serialize)]
pub enum DeviceEventKind {
    MousePress,
    MouseRelease,
    MouseMove,
    KeyboardPress,
    KeyboardRelease,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceEvent {
    kind: DeviceEventKind,
    value: Value,
}

static IS_LISTENING: AtomicBool = AtomicBool::new(false);

#[command]
pub async fn start_device_listening<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    if IS_LISTENING.load(Ordering::SeqCst) {
        return Ok(());
    }

    IS_LISTENING.store(true, Ordering::SeqCst);

    // 在新线程中运行监听事件，避免阻塞主线程
    std::thread::spawn(move || {
        let last_mouse_move_time = Arc::new(Mutex::new(Instant::now()));

        let throttle_duration = Duration::from_millis(30);

        let callback = move |event: Event| {
            let device_event = match event.event_type {
                EventType::ButtonPress(button) => DeviceEvent {
                    kind: DeviceEventKind::MousePress,
                    value: json!(format!("{:?}", button)),
                },
                EventType::ButtonRelease(button) => DeviceEvent {
                    kind: DeviceEventKind::MouseRelease,
                    value: json!(format!("{:?}", button)),
                },
                EventType::MouseMove { x, y } => {
                    // --- 节流 ---
                    let mut last_time = last_mouse_move_time.lock().unwrap();
                    if last_time.elapsed() < throttle_duration {
                        // 如果距离上次发送时间太短，直接忽略此次事件
                        return;
                    }
                    // 更新最后发送时间
                    *last_time = Instant::now();
                    // --- 鼠标移动节流逻辑结束 ---

                    DeviceEvent {
                        kind: DeviceEventKind::MouseMove,
                        value: json!({ "x": x, "y": y }),
                    }
                }
                EventType::KeyPress(key) => DeviceEvent {
                    kind: DeviceEventKind::KeyboardPress,
                    value: json!(format!("{:?}", key)),
                },
                EventType::KeyRelease(key) => DeviceEvent {
                    kind: DeviceEventKind::KeyboardRelease,
                    value: json!(format!("{:?}", key)),
                },
                _ => return,
            };

            let _ = app_handle.emit("device-changed", device_event);
        };

        // 在 thread::spawn 中 开始监听
        if let Err(error) = listen(callback) {
            eprintln!("Error: {:?}", error);

            IS_LISTENING.store(false, Ordering::SeqCst);
        }
    });

    Ok(())
}
