use e_imzo::list_all_certificates;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{
        self,
        prelude::{BoxExt, WidgetExt},
    },
    AsyncComponentSender, RelmWidgetExt,
};
use std::{
    collections::HashMap,
    fs, io,
    os::unix::fs::MetadataExt,
    path::Path,
    process::{Command, ExitStatus},
    time::Duration,
};

use crate::{
    config::LIBEXECDIR,
    pages::select_mode::{SelectModeMsg, SelectModePage},
};

pub fn is_service_active(service_name: &str) -> Result<bool, String> {
    let output = Command::new("systemctl")
        .args(&["--user", "is-active", service_name])
        .output()
        .map_err(|e| format!("Failed to run systemctl: {}", e))?;

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    match status.as_str() {
        "active" => Ok(true),
        "inactive" | "failed" | "activating" | "deactivating" | "unknown" => Ok(false),
        _ => Err(format!("Unexpected status: {}", status)),
    }
}

pub fn check_service_active(service: &str) -> bool {
    match is_service_active(service) {
        Ok(active) => active,
        Err(_e) => {
            // eprintln!("Error checking service: {}", e);
            false
        }
    }
}

pub fn get_pfx_files_in_folder() -> io::Result<Vec<String>> {
    let path = Path::new("/media/DSKEYS");
    let entries = fs::read_dir(path)?;

    let pfx_files: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension()?.to_str()? == "pfx" {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(pfx_files)
}

pub fn return_pfx_files_in_folder() -> Vec<String> {
    let mut certificate = Vec::<String>::new();

    let path = Path::new("/media/DSKEYS");
    if path.exists() {
        match get_pfx_files_in_folder() {
            Ok(file_names) => {
                for file_name in file_names {
                    certificate.push(file_name.clone());
                }
            }
            Err(e) => tracing::error!(
                "Error in Init function eimzo::get_pfx_files_in_folder: {}",
                e
            ),
        }
    }
    return certificate;
}

pub fn check_file_ownership() -> Result<u32, Box<dyn std::error::Error>> {
    let path = Path::new("/media/DSKEYS");
    let metadata = fs::metadata(path)?;
    let uid = metadata.uid();
    return Ok(uid);
}

pub fn check_service_installed(service: &str) -> bool {
    let path = Path::new(service);

    if path.exists() {
        return true;
    }

    false
}

// file selection filter .pfx file
pub fn tasks_filename_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::default();
    filename_filter.set_name(Some("PFX (.pfx)"));
    filename_filter.add_suffix("pfx");

    vec![filename_filter]
}

// bunch of gtk C style code in Rust
pub fn add_file_row_to_list(
    certificate: e_imzo::Certificate,
    alias: HashMap<String, String>,
    file_list: &adw::PreferencesGroup,
    sender: AsyncComponentSender<SelectModePage>,
) -> &adw::PreferencesGroup {
    // convert string "2027.07.23 17:44:06" into "23.07.2027"
    let validfrom: Vec<_> = alias.get("validfrom").unwrap().split(" ").collect();
    let mut validfrom_dmy: Vec<_> = validfrom[0].split(".").collect();
    validfrom_dmy.reverse();

    let validto: Vec<_> = alias.get("validto").unwrap().split(" ").collect();
    let mut validto_dmy: Vec<_> = validto[0].split(".").collect();
    validto_dmy.reverse();
    // all data from certificate
    let full_name = alias.get("cn").unwrap();
    let serial_number = alias.get("serialnumber").unwrap();
    let name = alias.get("name").unwrap();
    let surname = alias.get("surname").unwrap();
    let file_name = certificate.name;

    let full_name_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    full_name_box.set_margin_all(12);
    full_name_box.set_hexpand(true);
    full_name_box.append(
        &gtk::Label::builder()
            .label(gettext("Full name"))
            .css_classes(["dim-label"])
            .build(),
    );
    full_name_box.append(&gtk::Label::new(Some(&full_name.to_uppercase())));

    let serial_number_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    serial_number_box.set_margin_all(12);
    serial_number_box.set_hexpand(true);
    serial_number_box.append(
        &gtk::Label::builder()
            .label(gettext("Sertificate number:"))
            .css_classes(["dim-label"])
            .build(),
    );
    serial_number_box.append(&gtk::Label::new(Some(&serial_number.to_uppercase())));

    let valid_date_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    valid_date_box.set_margin_all(12);
    valid_date_box.set_hexpand(true);
    valid_date_box.append(
        &gtk::Label::builder()
            .label(gettext("Certificate validity period:"))
            .css_classes(["dim-label"])
            .build(),
    );
    valid_date_box.append(&gtk::Label::new(Some(&format!(
        "{} - {}",
        validfrom_dmy.join("."),
        validto_dmy.join(".")
    ))));

    let remove_button = gtk::Button::new();
    remove_button.set_label(&gettext("Delete"));
    remove_button.add_css_class("destructive-action");
    remove_button.set_align(gtk::Align::End);

    remove_button.connect_clicked(move |_| {
        show_remove_file_alert_dialog(file_name.clone(), sender.clone());
    });

    let expander = adw::ExpanderRow::builder()
        .title(format!(
            "<b>{} {}</b>",
            name.to_uppercase(),
            surname.to_uppercase()
        ))
        .use_markup(true)
        .build();
    expander.add_row(&adw::ActionRow::builder().child(&full_name_box).build());
    expander.add_row(&adw::ActionRow::builder().child(&serial_number_box).build());
    expander.add_row(&adw::ActionRow::builder().child(&valid_date_box).build());
    expander.add_row(&adw::ActionRow::builder().child(&remove_button).build());

    file_list.add(&expander);

    return file_list;
}

pub async fn refresh_certificates(
    file_list: &adw::PreferencesGroup,
    sender: AsyncComponentSender<SelectModePage>,
) -> &adw::PreferencesGroup {
    loop {
        match list_all_certificates() {
            Ok(pfx) => {
                pfx.iter()
                    .map(|c| (c, c.get_alias()))
                    .for_each(|(c, alias)| {
                        add_file_row_to_list(c.clone(), alias, file_list, sender.clone());
                    });
                break;
            }
            Err(e) => {
                tracing::info!("Waiting for service activation: {}", e);
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    return file_list;
}

// ask password if user has no permission to open /media/DSKEYS folder
pub fn ask_password(sender: AsyncComponentSender<SelectModePage>) -> () {
    relm4::spawn(async move {
        let output = tokio::process::Command::new("pkexec")
            .arg(format!("{}/e-helper", LIBEXECDIR))
            .output()
            .await;
        match output {
            Ok(output) => {
                if !ExitStatus::success(&output.status) {
                    // do nothing if user canceled entering password
                    return;
                }
                sender.input(SelectModeMsg::OpenFileConfirmed);
            }
            Err(e) => {
                eprintln!("Failed to execute pkexec: {}", e);
            }
        }
    });
}

pub fn show_alert_dialog(text: &str) {
    let dialog = adw::AlertDialog::builder()
        .heading(text)
        .default_response("ok")
        .follows_content_size(true)
        .build();

    dialog.add_responses(&[("ok", &gettext("OK"))]);

    dialog.connect_response(None, |dialog, response| {
        println!("Dialog response: {}", response);
        dialog.close();
    });

    if let Some(win) = relm4::main_application().active_window() {
        dialog.present(Some(&win));
    }
}

pub fn show_remove_file_alert_dialog(file_name: String, sender: AsyncComponentSender<SelectModePage>) -> () {
    let dialog = adw::AlertDialog::builder()
        .heading("Are you sure?")
        .body("Do you really want to delete this certificate?")
        .build();

    dialog.add_responses(&[("yes", "Yes"), ("no", "No")]);
    dialog.set_default_response(Some("no"));

    dialog.set_response_appearance("yes", adw::ResponseAppearance::Destructive);
    dialog.set_response_appearance("no", adw::ResponseAppearance::Suggested);

    dialog.connect_response(None, {
        let sender = sender.clone();
        let file_name = file_name.clone();
        move |dialog, response| {
            match response {
                "yes" => {
                    sender.input(SelectModeMsg::RemoveCertificates(file_name.clone()));
                }
                "no" => {
                    sender.input(SelectModeMsg::RefreshCertificates);
                }
                _ => {}
            }
            dialog.close();
        }
    });
    if let Some(win) = relm4::main_application().active_window() {
        dialog.present(Some(&win));
    }
}
