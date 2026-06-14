#[derive(QObject, Default)]
pub struct PinnedModel {
    base: qt_base_class!(trait QAbstractListModel),

    apps: Vec<AppEntry>,
}

impl QAbstractListModel for PinnedModel {
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
            PATH_ROLE => self.apps[row].path.clone().into(),
            _ => QVariant::default(),
        }
    
    }

    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {

        let mut roles =
            std::collections::HashMap::new();

        roles.insert(
            USER_ROLE,
            QByteArray::from("name")
        );

        roles.insert(
            ICON_ROLE,
            QByteArray::from("icon")
        );

        roles.insert(
            PATH_ROLE,
            QByteArray::from("path")
        );

        roles
    }
}

impl PinnedModel {
    fn set_apps(&mut self, apps: Vec<AppEntry>) {
        self.begin_reset_model();

        self.apps = apps;

        self.end_reset_model();
    }
}