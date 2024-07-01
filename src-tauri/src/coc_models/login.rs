use crate::api::Client;
use crate::credentials::CredentialsBuilder;
use crate::Response;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::create_dir_all;
use std::sync::Mutex;
use tauri::{
    api::path::{resolve_path, BaseDirectory},
    AppHandle, Manager,
};
use tokio::sync::OnceCell;

#[derive(Deserialize, Serialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
    pub remember: bool,
}

pub static CLIENT: OnceCell<Mutex<Option<Client>>> = OnceCell::const_new();

#[tauri::command]
pub async fn login(app: AppHandle, login_info: LoginInfo) -> Result<Response, Response> {
    let credentials = CredentialsBuilder::default()
        .add_credential(login_info.email.clone(), login_info.password.clone())
        .build();

    match Client::new(credentials).await {
        Ok(client) => {
            CLIENT
                .get_or_init(|| async { Mutex::new(Some(client)) })
                .await;

            let login_info_path = resolve_path(
                &app.config(),
                app.package_info(),
                &app.env(),
                "login_info.json",
                Some(BaseDirectory::AppConfig),
            )
            .expect("Failed to resolve path");

            if login_info.remember {
                let saved_login = LoginInfo {
                    email: login_info.email,
                    password: login_info.password,
                    remember: login_info.remember,
                };

                // Create parent directories if they don't exist
                if let Some(parent) = login_info_path.parent() {
                    create_dir_all(parent).expect("Failed to create parent directories");
                }

                fs::write(
                    &login_info_path,
                    serde_json::to_string(&saved_login).unwrap(),
                )
                .expect("Failed to write login info");
            } else {
                if fs::metadata(&login_info_path).is_ok() {
                    fs::remove_file(&login_info_path).expect("Failed to remove login info");
                }
            }

            Ok(Response {
                status: 200,
                message: "Login successful".to_string(),
            })
        }
        Err(e) => Err(Response {
            status: 400,
            message: e.to_string(),
        }),
    }
}

pub async fn get_client() -> Result<Client, ()> {
    match {
        let client_guard = CLIENT.get().unwrap().lock();
        match client_guard {
            Ok(guard) => guard.clone(),
            Err(_) => {
                println!("Client is poisoned.");
                return Err(());
            }
        }
    } {
        Some(client) => Ok(client),
        None => {
            println!("Client is not initialized.");
            Err(())
        }
    }
}
