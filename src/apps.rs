use qmetaobject::*;

#[derive(Clone, Default)]
pub struct AppEntry {
    pub name: QString,
}

#[derive(QObject)]
pub struct AppModel {
    base: qt_base_class!(trait QAbstractListModel),

    apps: Vec<AppEntry>,

    name: qt_property!(QString; CONST),
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            base: Default::default(),

            apps: vec!{
                AppEntry {
                    name: "Firefox".into(),
                },
                AppEntry {
                    name: "VS Code".into(),
                },
                AppEntry {
                    name: "Discord".into(),
                },
                AppEntry {
                    name: "Terminal".into(),
                },
                AppEntry {
                    name: "Spotify".into(),
                },
            },

            name: QString::from("apps"),
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
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
        let mut roles = std::collections::HashMap::new();

        roles.insert(USER_ROLE, QByteArray::from("name"));

        roles
    }
}