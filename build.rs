// This tells tonic-build to comple the proto buffer when running cargo build/run

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/job.proto")?;
    Ok(())
}
