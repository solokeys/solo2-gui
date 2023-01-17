use tauri::Window;
use solo2::{Device, UuidSelectable};

#[derive(Clone, serde::Serialize)]
struct Prompt {
    message: String
}

#[tauri::command(async)]
pub async fn update_device(window: Window, uuid: String) -> Result<String, String> {
    // let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    // let reply_listener = window.listen("reply", move |event| {
    //     let payload = event.payload().unwrap().to_string();
    //     tx.send(payload).ok();
    // });
    let uuid = uuid.parse()
        .map_err(|_| "error parsing UUID")?;

    let firmware: solo2::Firmware = solo2::Firmware::download_latest()
        .map_err(|_| "could not download latest firmware")?;

    let calver = firmware.version().to_calver();

    println!(
        "Fetched firmware version {} ({})",
        &calver,
        &firmware.version().to_semver(),
    );

    window.emit("firmware-size", firmware.len()).ok();
    // window.emit(
    //     "prompt",
    //     Prompt { message: format!("Updating to firmware version {calver}") }
    // ).unwrap();
    // println!("Sent prompt");
    // let response = rx.recv().await.unwrap();
    // println!("response: {:?}", response);
    // window.unlisten(reply_listener);
    // if response != "ok" {
    //     println!("prompt not OK, returning");
    //     return Ok("".to_string());
    // }

    use tauri::api::dialog::blocking::confirm;
    if !confirm(
        Some(&window),
        "Confirm firmware version",
        format!("Updating to firmware version {calver}\nIf you proceed, you will need to touch the device next.")
    ) {
        println!("prompt not OK, returning");
        return Ok("".to_string());
    }

    let device = solo2::Device::having(uuid)
        .map_err(|_| "no such device")?;
    let bar = indicatif::ProgressBar::new(firmware.len() as u64);
    let progress = |position: usize| {
        window.emit("firmware-position", position).ok();
        bar.set_position(position as u64);
    };

    window.emit("touch-device", true).ok();
    if let Ok(device) = device.into_lpc55() {
        window.emit("update-starts", "").ok();
        let yes = false;
        return Device::Lpc55(device).program(firmware, yes, Some(&progress))
            .map(|_| calver)
            .map_err(|_| "failed to update".to_string());
    } else {
        use tauri::api::dialog::message;
        window.emit("touch-device", false).ok();
        message(Some(&window), "Failure", "Failed to switch to maintenance mode.");
        return Err("Failed to switch to maintenance mode!".into());
    }
}

