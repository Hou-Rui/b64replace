use b64replace_core::Base64Replacer;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, template, READ, WRITE, NOTIFY = text_changed)]
        #[qproperty(QString, input, READ, WRITE, NOTIFY = text_changed)]
        #[qproperty(QString, output, READ = get_output, NOTIFY = text_changed)]
        #[qproperty(QString, error)]
        type Backend = super::BackendRust;
        #[qsignal]
        fn text_changed(self: Pin<&mut Self>);
        fn get_output(self: Pin<&mut Self>) -> QString;
    }
}

use cxx_qt_lib::QString;
use std::{
    io::{BufReader, BufWriter, Cursor},
    pin::Pin,
};
use anyhow::Result;

#[derive(Default)]
pub struct BackendRust {
    input: QString,
    template: QString,
    error: QString,
}

impl ffi::Backend {
    pub fn get_output(self: Pin<&mut Self>) -> QString {
        let run = || -> Result<String> {
            let mut input = BufReader::new(Cursor::new(self.input.to_string()));
            let mut output = BufWriter::new(Vec::new());
            let template = self.template.to_string();
            let replacer = Base64Replacer::new(template);
            replacer.replace(&mut input, &mut output, &mut |_| {})?;
            let decoded = output.into_inner()?;
            Ok(String::from_utf8(decoded)?)
        };

        match run() {
            Ok(output) => {
                self.set_error(QString::from(""));
                output.into()
            }
            Err(err) => {
                self.set_error(QString::from(format!("{}", err)));
                QString::from("")
            }
        }
    }
}
