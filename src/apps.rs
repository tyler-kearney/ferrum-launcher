use crate::config::{Config, load_config, save_config};

use qmetaobject::*;

use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

const ICON_ROLE: i32 = USER_ROLE + 1;
const PINNED_ROLE: i32 = USER_ROLE + 2;
const PATH_ROLE: i32 = USER_ROLE + 3;

#[derive(Clone, Default)]
pub struct AppEntry {
    pub name: QString,
    pub path: QString,
    pub icon: QString,
    pub pinned: bool,
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

                        let path_string = path.to_string_lossy().to_string();

                        let icon_path = get_app_icon(&path);

                        apps.push(AppEntry {
                            name: name.into(),
                            path: path_string.clone().into(),
                            icon: icon_path.into(),
                            pinned: false,
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
    filtered_apps: Vec<AppEntry>,

    name: qt_property!(QString; CONST),

    launch_app: qt_method!(fn(&mut self, app_name: QString)),

    search_text: QString,

    set_search: qt_method!(fn(&mut self, text: QString)),

    pinned_apps: HashSet<String>,

    toggle_pin: qt_method!(fn(&mut self, app_path: QString)),

    pinned_entries: Vec<AppEntry>,
}

impl Default for AppModel {
    fn default() -> Self {
        let mut apps = scan_applications();

        let config = load_config();

        let pinned_apps: HashSet<String> = config.pinned_apps.iter().cloned().collect();

        for app in &mut apps {
            app.pinned = pinned_apps.contains(&app.path.to_string());
        }

        let pinned_entries = apps
            .iter()
            .filter(|app| app.pinned)
            .cloned()
            .collect();

        Self {
            base: Default::default(),

            apps: apps.clone(),
            filtered_apps: apps,

            name: QString::from("apps"),

            launch_app: Default::default(),

            search_text: QString::from(""),

            set_search: Default::default(),

            pinned_apps,

            toggle_pin: Default::default(),

            pinned_entries,
        }
    }
}

impl QAbstractListModel for AppModel {
    fn row_count(&self) -> i32 {
        self.filtered_apps.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let row = index.row() as usize;

        if row >= self.filtered_apps.len() {
            return QVariant::default();
        }

        match role {
            USER_ROLE => self.filtered_apps[row].name.clone().into(),
            ICON_ROLE => self.filtered_apps[row].icon.clone().into(),
            PINNED_ROLE => self.filtered_apps[row].pinned.into(),
            PATH_ROLE => self.filtered_apps[row].path.clone().into(),
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
        let mut roles = std::collections::HashMap::new();

        roles.insert(USER_ROLE, QByteArray::from("name"));

        roles.insert(ICON_ROLE, QByteArray::from("icon"));

        roles.insert(PINNED_ROLE, QByteArray::from("pinned"));

        roles.insert(PATH_ROLE, QByteArray::from("path"));

        roles
    }
}

impl AppModel {
    fn set_search(&mut self, text: QString) {
        self.search_text = text.clone();

        let query = text.to_string().to_lowercase();

        self.begin_reset_model(); // Lets Qt know that we're about to replace the model contents

        self.filtered_apps = self
            .apps
            .iter()
            .filter(|app| {
                app.name
                    .to_string()
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect();

        // Let Qt know that we're done resetting the app model
        self.end_reset_model();

        println!(
            "Search '{}' found {} apps",
            query,
            self.filtered_apps.len()
        );
    }

    fn save_pins(&self) {
        let config = Config {
            pinned_apps: self
                .pinned_apps
                .iter()
                .cloned()
                .collect(),
        };

        save_config(&config);
    }

    fn rebuild_pinned_entries(&mut self) {
        self.pinned_entries = self
            .apps
            .iter()
            .filter(|app| app.pinned)
            .cloned()
            .collect();
    }

    fn toggle_pin(&mut self, app_path: QString) {
        let path = app_path.to_string();

        if self.pinned_apps.contains(&path) {
            self.pinned_apps.remove(&path);
        } else {
            self.pinned_apps.insert(path.clone());
        }

        for app in &mut self.apps {
            if app.path.to_string() == path {
                app.pinned = !app.pinned;
                break;
            }
        }

        for app in &mut self.filtered_apps {
            if app.path.to_string() == path {
                app.pinned = !app.pinned;
                break;
            }
        }

        self.rebuild_pinned_entries();

        self.save_pins();

        println!("Pinned count: {}", self.pinned_apps.len());
    }

    fn pinned_entries(&self) -> Vec<AppEntry> {
        self.apps
            .iter()
            .filter(|app| app.pinned)
            .cloned()
            .collect()
    }

    fn launch_app(&mut self, app_name: QString) {
        let app = app_name.to_string();

        println!("Launching {}", app);

        let _ = std::process::Command::new("open")
            .arg("-a")
            .arg(&app)
            .spawn();
    }
}

