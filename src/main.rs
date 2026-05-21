mod apps;

use apps::AppModel;
use qmetaobject::*;

fn main() {
    qml_register_type::<AppModel>(
        cstr::cstr!("Ferrum"),
        1,
        0,
        cstr::cstr!("AppModel")
    );

    let mut engine = QmlEngine::new();

    engine.load_file("qml/Main.qml".into());

    engine.exec();
}

/* 
#[derive(QObject, Default)]
struct SimpleLogger {
    base: qt_base_class!(trait QObject),
}
    */