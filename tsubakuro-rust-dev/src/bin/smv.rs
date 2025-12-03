use tsubakuro_rust_core::prelude::{
    CoreClient, EndpointBrokerClient, ServiceMessageVersion, SqlClient, WireClient,
};

fn main() {
    let wire = WireClient::service_message_version();
    println!("Wire SMV: {}", wire);
    let core = CoreClient::service_message_version();
    println!("Core SMV: {}", core);
    let broker = EndpointBrokerClient::service_message_version();
    println!("EndpointBroker SMV: {}", broker);
    let sql = SqlClient::service_message_version();
    println!("SqlClient SMV: {}", sql);
}
