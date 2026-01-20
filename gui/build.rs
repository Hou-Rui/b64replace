use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("b64replace").qml_file("qml/Main.qml"))
        .qt_module("Network")
        .qt_module("Widgets")
        .files(["src/backend.rs"])
        .build();
}
