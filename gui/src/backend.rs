use b64replace_core::Base64Replacer;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Backend = super::BackendRust;

        #[qinvokable]
        #[cxx_name = "replaceAll"]
        pub fn replace_all(self: Pin<&mut Self>, input: QString) -> QString;
    }
}

use cxx_qt_lib::QString;
use std::{
    io::{BufReader, BufWriter, Cursor},
    pin::Pin,
};

#[derive(Default)]
pub struct BackendRust;

impl ffi::Backend {
    pub fn replace_all(self: Pin<&mut Self>, input: QString) -> QString {
        let input = BufReader::new(Cursor::new(input.to_string()));
        let mut output = BufWriter::new(Vec::new());
        let mut replacer = Base64Replacer::new(String::from("^{}$"));
        replacer
            .replace_all(input, &mut output)
            .expect("Failed to replace all");
        let decoded = output.into_inner().expect("Failed to unpack writer");
        String::from_utf8(decoded).unwrap().into()
    }
}
