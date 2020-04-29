use notify_rust::Notification;

pub fn show_notif<M: Into<String>>(message: M) {

    drop(
        Notification::new()
        .summary("Tempfiles Upload")
        .icon("system-file-manager")
        .body(&message.into())
        .show()
    );

}

pub fn fix_filename(filename: String) -> String {

    percent_encoding::percent_decode_str(&filename)
        .decode_utf8_lossy()
        .to_string()
        .replace("file://", "")

}
