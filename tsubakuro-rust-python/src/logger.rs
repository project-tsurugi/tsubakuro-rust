use std::sync::Once;

use env_logger::{Builder, Target};
use log::trace;

static ENV_LOGGER_INIT: Once = Once::new();

pub fn env_logger_init(filters: &str, file_path: Option<String>) {
    const FUNCTION_NAME: &str = "env_logger_init()";
    trace!("{FUNCTION_NAME} start");

    let filters = filters.replace("tsubakuro_rust_python", "_tsubakuro_rust_python");
    let filters = filters.replace("__tsubakuro_rust_python", "_tsubakuro_rust_python");

    ENV_LOGGER_INIT.call_once(|| {
        let mut builder = Builder::new();

        builder.parse_filters(&filters);

        if let Some(file_path) = file_path {
            env_logger_init_file(&mut builder, &file_path);
        }

        builder.format_timestamp_millis().init();
    });

    trace!("{FUNCTION_NAME} end");
}

fn env_logger_init_file(builder: &mut Builder, file_path: &str) {
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
    {
        Ok(file) => {
            builder.target(Target::Pipe(Box::new(file)));
        }
        Err(e) => {
            eprintln!("env_logger_init(): log file open error. {}", e);
        }
    };
}
