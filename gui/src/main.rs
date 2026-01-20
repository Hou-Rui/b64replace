pub mod backend;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QQmlApplicationEngine, QQmlEngine, QUrl};
use cxx_qt_lib_extras::QApplication;
use std::pin::Pin;

fn main() {
    // Create the application and engine
    let mut app = QApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/b64replace/qml/Main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        let engine: Pin<&mut QQmlEngine> = engine.upcast_pin();
        // Listen to a signal from the QML Engine
        engine
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
