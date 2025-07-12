use std::sync::Once;

use env_logger::{Builder, Target};
use log::trace;

use crate::DRIVER_NAME;

static ENV_LOGGER_INIT: Once = Once::new();

pub(crate) fn env_logger_init() {
    ENV_LOGGER_INIT.call_once(|| {
        const FUNCTION_NAME: &str = "env_logger_init()";
        trace!("{FUNCTION_NAME} start");

        let _ = env_logger_init_main();

        trace!("{FUNCTION_NAME} end");
    })
}

fn env_logger_init_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Builder::new();

    if let Ok(filters) = std::env::var("TSURUGI_ODBC_DRIVER_LOGLEVEL") {
        builder.parse_filters(&filters);
    } else {
        return Ok(());
    }

    if let Ok(path) = std::env::var("TSURUGI_ODBC_DRIVER_LOGFILE") {
        env_logger_init_file(&mut builder, &path)?
    }

    builder.format_timestamp_millis();

    if let Err(e) = builder.try_init() {
        eprintln!("{DRIVER_NAME}: env_logger init fail. {}", e);
        return Err(e.into());
    }

    Ok(())
}

fn env_logger_init_file(
    builder: &mut Builder,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{DRIVER_NAME}: log file open error. {}", e);
            return Err(e.into());
        }
    };

    builder.target(Target::Pipe(Box::new(file)));
    Ok(())
}
