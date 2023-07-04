use std::path::Path;

use anyhow::Context;
use log::info;

use crate::{
  config::LauncherConfig,
  util::{
    file::{create_dir, delete_dir},
    network::download_file,
    os::open_dir_in_os,
    tar::extract_and_delete_tar_ball,
    zip::extract_and_delete_zip_file,
  },
};

use super::CmdErr;

#[tauri::command]
pub async fn list_downloaded_versions(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
) -> Result<Vec<String>, CmdErr> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok(Vec::new()),
    Some(path) => Path::new(path),
  };

  let expected_path = Path::new(install_path)
    .join("versions")
    .join(version_folder);
  if !expected_path.exists() || !expected_path.is_dir() {
    log::info!(
      "Folder '{}' not found, returning empty version list",
      expected_path.display()
    );
    return Ok(vec![]);
  }

  let entries = std::fs::read_dir(&expected_path).context("Unable to read versions folder")?;

  Ok(
    entries
      .filter_map(|e| {
        e.ok().and_then(|d| {
          let p = d.path();
          if p.is_dir() {
            Some(
              p.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or("".into()),
            )
          } else {
            None
          }
        })
      })
      .collect(),
  )
}

#[tauri::command]
pub async fn download_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version: String,
  version_folder: String,
  url: String,
) -> Result<(), CmdErr> {
  let config_lock = config.lock().await;
  let install_path = config_lock
    .installation_dir
    .as_ref()
    .map(|p| Path::new(p))
    .context("Cannot download version, no installation directory set")?;

  let dest_dir = install_path
    .join("versions")
    .join(&version_folder)
    .join(&version);

  log::info!(
    "Downloading version '{}' to '{}'",
    version,
    dest_dir.display()
  );

  // Delete the directory if it exists, and create it from scratch
  delete_dir(&dest_dir).context("Unable to delete destination folder for download")?;
  create_dir(&dest_dir).context("Unable to create destination folder for download")?;

  let (filename, extractor) = if cfg!(windows) {
    (format!("{version}.zip"), "extractor.exe")
  } else if cfg!(unix) {
    (format!("{version}.tar.gz"), "extractor")
  } else {
    return Err(CmdErr::new(
      "Unknown operating system, unable to download and extract correct release",
    ));
  };

  let download_path = install_path
    .join("versions")
    .join(&version_folder)
    .join(&filename);

  download_file(&url, &download_path)
    .await
    .context("Unable to download version")?;

  match &filename {
    f if f.ends_with(".zip") => extract_and_delete_zip_file(&download_path, &dest_dir)
      .context("Unable to extract downloaded version")?,
    f if f.ends_with(".tar.gz") => extract_and_delete_tar_ball(&download_path, &dest_dir)
      .context("Unable to extract downloaded version")?,
    _ => {
      return Err(CmdErr::new(
        "Unknown file type, unable to extract downloaded version",
      ))
    }
  };

  let expected_extractor_path = dest_dir.join(&extractor);
  if !expected_extractor_path.exists() {
    log::info!(
      "Version did not extract properly, {} is missing!",
      expected_extractor_path.display()
    );
    delete_dir(&dest_dir).context("Unable to delete bad version folder")?;
    return Err(CmdErr::new(
      "Version did not extract properly, critical files are missing. An antivirus may have deleted the files!".to_owned(),
    ));
  }

  Ok(())
}

#[tauri::command]
pub async fn remove_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version: String,
  version_folder: String,
) -> Result<(), CmdErr> {
  let mut config_lock = config.lock().await;
  let install_path = config_lock
    .installation_dir
    .as_ref()
    .map(|p| Path::new(p))
    .ok_or(CmdErr::new(
      "Cannot remove version, no installation directory set",
    ))?;

  info!("Deleting Version '{}' from '{}'", version, version_folder);

  let version_dir = install_path
    .join("versions")
    .join(&version_folder)
    .join(&version);

  delete_dir(&version_dir).context("Unable to delete version directory")?;

  // If it's the active version, we should clean that up in the settings file
  if let (Some(config_version_folder), Some(config_version)) = (
    &config_lock.active_version_folder,
    &config_lock.active_version,
  ) {
    if (version_folder == *config_version_folder) && (version == *config_version) {
      config_lock
        .clear_active_version()
        .context("Unable to clear active version from config")?;
    }
  }

  Ok(())
}

#[tauri::command]
pub async fn go_to_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
) -> Result<(), CmdErr> {
  let config_lock = config.lock().await;
  let install_path = config_lock
    .installation_dir
    .as_ref()
    .map(|p| Path::new(p))
    .ok_or_else(|| CmdErr::new("No installation directory set"))?;

  let folder_path = Path::new(install_path)
    .join("versions")
    .join(version_folder);
  create_dir(&folder_path).with_context(|| {
    format!(
      "Unable to create version folder '{}' in order to open it",
      folder_path.display()
    )
  })?;

  open_dir_in_os(folder_path.to_string_lossy().into_owned())
    .context("Unable to open folder in OS")?;

  Ok(())
}

#[tauri::command]
pub async fn ensure_active_version_still_exists(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CmdErr> {
  let mut config_lock = config.lock().await;
  let install_path = config_lock
    .installation_dir
    .as_ref()
    .map(|p| Path::new(p))
    .context("Unable to check if active version still exists, no installation directory set")?;

  info!(
    "Checking if active version still exists {:?}:{:?}",
    config_lock.active_version_folder, config_lock.active_version
  );

  return Err(CmdErr::new("test error, please ignore"));

  let active_version_folder = config_lock.active_version_folder.as_ref();
  let active_version = config_lock.active_version.as_ref();

  if active_version_folder.is_none() || active_version.is_none() {
    return Ok(false);
  }

  let version_dir = install_path
    .join("versions")
    .join(active_version_folder.unwrap())
    .join(active_version.unwrap());

  let version_exists = version_dir.exists();

  if !version_exists {
    // Clear active version if it's no longer available
    config_lock
      .clear_active_version()
      .context("Unable to clear active version from config")?;
  }

  Ok(version_exists)
}
