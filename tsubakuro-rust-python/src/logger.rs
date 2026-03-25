use std::sync::Once;

use env_logger::{Builder, Target};
use log::{trace, LevelFilter};
use pyo3::{exceptions::PyRuntimeError, PyResult, Python};

static LOGGER_INIT: Once = Once::new();

pub fn env_logger_init(filters: &str, file_path: Option<String>) {
    const FUNCTION_NAME: &str = "env_logger_init()";
    trace!("{FUNCTION_NAME} start");

    let filters = filters.replace("tsubakuro_rust_python", "_tsubakuro_rust_python");
    let filters = filters.replace("__tsubakuro_rust_python", "_tsubakuro_rust_python");

    LOGGER_INIT.call_once(|| {
        let mut builder = Builder::new();

        builder.parse_filters(&filters);

        if let Some(file_path) = file_path {
            env_logger_init_file(&mut builder, &file_path);
        }

        builder.format_timestamp_millis().init();

        trace!("{FUNCTION_NAME} initialized");
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

pub fn logging_init(py: Python) {
    const FUNCTION_NAME: &str = "logging_init()";
    trace!("{FUNCTION_NAME} start");

    LOGGER_INIT.call_once(|| {
        let _ = pyo3_log_init_main(py);
        trace!("{FUNCTION_NAME} initialized");
    });

    trace!("{FUNCTION_NAME} end");
}

fn pyo3_log_init_main(py: Python) -> PyResult<()> {
    use pyo3_log::{Caching, Logger};

    Logger::new(py, Caching::LoggersAndLevels)?
        .filter(LevelFilter::Trace)
        .install()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    Ok(())
}
