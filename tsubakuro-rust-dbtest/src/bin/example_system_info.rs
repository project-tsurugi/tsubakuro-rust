use std::time::Duration;

use tsubakuro_rust_core::prelude::*;

#[tokio::main]
async fn main() -> Result<(), TgError> {
    example().await
}

async fn example() -> Result<(), TgError> {
    let endpoint = Endpoint::parse("tcp://localhost:12345")?;

    let mut connection_option = ConnectionOption::new();
    connection_option.set_endpoint(endpoint);
    connection_option.set_application_name("Tsubakuro/Rust example");
    connection_option.set_session_label("example session");
    connection_option.set_default_timeout(Duration::from_secs(10));

    // Session生成
    let session = Session::connect(&connection_option).await?;

    // SystemClient生成
    let client: SystemClient = session.make_client();

    // システム情報取得
    let system_info = client.get_system_info().await?;
    println!("name: {}", system_info.name());
    println!("version: {}", system_info.version());

    // Sessionクローズ
    session.close().await?;
    Ok(())
}
