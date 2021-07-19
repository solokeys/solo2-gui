use solo2::apps::App as _;

#[tauri::command]
fn get_uuid() -> Result<String, String> {
    use solo2::apps::admin::App;
    let mut app = App::new()
        .map_err(|_| "could not construct admin app")?;
    app.select().map_err(|_| "could not select admin app")?;
    let uuid = app.uuid().map_err(|_| "could not get a UUID")?;
    Ok(hex::encode_upper(uuid.to_be_bytes()))
}

#[tauri::command]
fn register_totp(user: String, secret: String) -> Result<String, String> {
    use solo2::apps::oath::{App, Credential};
    let credential = Credential::with_label_and_secret(&user, &secret)
        .map_err(|_| "could not parse secret")?;

    let mut app = App::new()
        .map_err(|_| "could not construct OATH app")?;
    app.select()
        .map_err(|_| "could not select OATH app")?;


    let label = app.register(credential)
        .map_err(|_| "could not register credential")?;
    dbg!(&label);

    Ok(label)

}

#[tauri::command]
fn calculate_totp(label: String) -> Result<String, String> {
    use solo2::apps::oath::{App, Authenticate};

    let mut app = App::new()
        .map_err(|_| "could not construct OATH app")?;
    app.select()
        .map_err(|_| "could not select OATH app")?;

    let authenticate = Authenticate::with_label(&label);
    let code = app.authenticate(authenticate)
        .map_err(|_| "could not authenticate")?;

    dbg!(&code);
    Ok(code)
}

#[cfg(test)]
mod tests {
}
