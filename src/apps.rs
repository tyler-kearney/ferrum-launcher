use qmetaobject::*;

use std::fs;
use std::path::PathBuf;

const ICON_ROLE: i32 = USER_ROLE + 1;

#[derive(Clone, Default)]
pub struct AppEntry {
    pub name: QString,
    pub path: QString,
    pub icon: QString,
}

fn get_app_icon(app_path: &PathBuf) -> String {
    let resources = app_path.join("Contents").join("Resources");

    if let Ok(entries) = fs::read_dir(&resources) {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "icns" {
                    return path.to_string_lossy().to_string();
                }
            }
        }
    }

    String::new()
}

fn scan_applications() -> Vec<AppEntry> {
    let mut apps = Vec::new();

    let mut search_paths = vec![PathBuf::from("/Applications")];

    if let Some(home) = std::env::var_os("HOME") {
        search_paths.push(PathBuf::from(home).join("Applications"));
    }

    for dir in search_paths {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Some(ext) = path.extension() {
                    if ext == "app" {
                        let name = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        let icon_path = get_app_icon(&path);

                        apps.push(AppEntry {
                            name: name.into(),
                            path: path.to_string_lossy().to_string().into(),
                            icon: icon_path.into(),
                        });
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| {
        a.name.to_string().cmp(&b.name.to_string())
    });

    apps
}

#[derive(QObject)]
pub struct AppModel {
    base: qt_base_class!(trait QAbstractListModel),

    apps: Vec<AppEntry>,

    name: qt_property!(QString; CONST),

    launch_app: qt_method!(fn(&mut self, app_name: QString)),
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            base: Default::default(),

            apps: scan_applications(),

            name: QString::from("apps"),

            launch_app: Default::default(),
        }
    }
}

impl QAbstractListModel for AppModel {
    fn row_count(&self) -> i32 {
        self.apps.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let row = index.row() as usize;

        if row >= self.apps.len() {
            return QVariant::default();
        }

        match role {
            USER_ROLE => self.apps[row].name.clone().into(),
            ICON_ROLE => self.apps[row].icon.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
        let mut roles = std::collections::HashMap::new();

        roles.insert(USER_ROLE, QByteArray::from("name"));

        roles.insert(ICON_ROLE, QByteArray::from("icon"));

        roles
    }
}

impl AppModel {
    fn launch_app(&mut self, app_name: QString) {
        let app = app_name.to_string();

        println!("Launching {}", app);

        let _ = std::process::Command::new("open")
            .arg("-a")
            .arg(&app)
            .spawn();
    }
}