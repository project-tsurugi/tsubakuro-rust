extern crate embed_resource;

fn main() {
    let macros = &[
        format!("REPLACE_VERSION=\"{}\"", env!("CARGO_PKG_VERSION")),
        format!(
            "REPLACE_VERSION_RAW={},{},{},0",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH")
        ),
    ];
    embed_resource::compile("resources/versioninfo.rc", macros)
        .manifest_optional()
        .unwrap();
}
