extern crate prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(
        &[
            "protos/jogasaki/proto/sql/common.proto",
            "protos/jogasaki/proto/sql/error.proto",
            "protos/jogasaki/proto/sql/request.proto",
            "protos/jogasaki/proto/sql/response.proto",
            "protos/tateyama/proto/core/request.proto",
            "protos/tateyama/proto/core/response.proto",
            "protos/tateyama/proto/diagnostics.proto",
            "protos/tateyama/proto/endpoint/request.proto",
            "protos/tateyama/proto/endpoint/response.proto",
            "protos/tateyama/proto/framework/common.proto",
            "protos/tateyama/proto/framework/request.proto",
            "protos/tateyama/proto/framework/response.proto",
            "protos/tateyama/proto/system/diagnostic.proto",
            "protos/tateyama/proto/system/request.proto",
            "protos/tateyama/proto/system/response.proto",
            "protos/tateyama/proto/system/request.proto",
            "protos/tateyama/proto/system/response.proto",
            "protos/tateyama/proto/blob_relay_privilege/request.proto",
            "protos/tateyama/proto/blob_relay_privilege/response.proto",
        ],
        &["protos/"],
    )?;
    tonic_prost_build::configure().compile_protos(
        &[
            "protos/data_relay_grpc/proto/blob_relay/blob_reference.proto",
            "protos/data_relay_grpc/proto/blob_relay/blob_relay_streaming.proto",
        ],
        &["protos/"],
    )?;
    Ok(())
}
