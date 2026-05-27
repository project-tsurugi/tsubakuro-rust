use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = _tsubakuro_rust_python::stub_info()?;
    stub.generate()?;
    Ok(())
}
