use solo2::{Select as _, Solo2, Uuid, UuidSelectable};
pub mod update;

fn solo2_from_uuid(uuid: &str) -> Result<Solo2, String> {
    let uuid2 = Uuid::parse_str(&uuid)
        .map_err(|_| format!("invalid UUID {}", &uuid))?;
    let solo2 = Solo2::having(uuid2)
        .map_err(|_| format!("no Solo2 found with UUID {}", &uuid))?;
    Ok(solo2)
}

#[tauri::command]
pub fn get_uuids() -> Result<Vec<String>, String> {

    Ok(Solo2::list()
        .iter()
        .map(|solo2| format!("{:X}", solo2.uuid().simple()).to_string())
        // .map(|solo2| format!("{}", &solo2))
        .collect())
}

// #[tauri::command]
// pub fn get_uuid() -> Result<String, String> {
//     use solo2::apps::Admin;

//     let mut solo2s = Solo2::list();
//     let solo2 = solo2s.get_mut(0)
//         .ok_or("could not find a Solo 2 device".to_string())?;
//     let mut app = Admin::select(solo2)
//         .map_err(|_| "could not select admin app")?;
//     // app.select().map_err(|_| "could not select admin app")?;
//     let uuid = app.uuid().map_err(|_| "could not get a UUID")?;
//     Ok(format!("{:X}", uuid))
// }

#[tauri::command]
pub fn wink(uuid: String) -> Result<String, String> {
    use solo2::apps::Admin;

    let mut solo2 = solo2_from_uuid(&uuid)?;
    let mut app = Admin::select(&mut solo2)
        .map_err(|_| "could not select admin app")?;
    app.wink().map_err(|_| "could not wink device")?;
    Ok("".into())
}

#[tauri::command]
pub fn register_totp(uuid: String, label: String, secret: String) -> Result<String, String> {
    use solo2::apps::Oath;

    let mut solo2 = solo2_from_uuid(&uuid)?;
    let mut app = Oath::select(&mut solo2).map_err(|_| "could not select OATH app")?;

    let credential = solo2::apps::oath::Credential::default_totp(&label, &secret)
        .map_err(|_| "could not parse secret")?;

    let label = app.register(credential)
        .map_err(|_| "could not register credential")?;

    Ok(label)
}

#[tauri::command]
pub fn list_totp(uuid: String) -> Result<Vec<String>, String> {
    use solo2::apps::Oath;

    let mut solo2 = solo2_from_uuid(&uuid)?;
    let mut app = Oath::select(&mut solo2).map_err(|_| "could not select OATH app")?;
    let labels = app.list().map_err(|_| "could not list labels")?;

    Ok(labels)
}

#[tauri::command]
pub fn calculate_totp(uuid: String, label: String) -> Result<String, String> {
    use solo2::apps::Oath;

    let mut solo2 = solo2_from_uuid(&uuid)?;
    let mut app = Oath::select(&mut solo2).map_err(|_| "could not select OATH app")?;

    let code = app.authenticate(solo2::apps::oath::Authenticate::with_label(&label))
        .map_err(|_| format!("could not get TOTP code for label {}", &label))?;

    Ok(code)
}

#[cfg(test)]
mod tests {
}
