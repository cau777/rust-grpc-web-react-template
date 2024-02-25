use tonic_build;

fn main() {
    // Add all your .proto files here
    tonic_build::compile_protos("./proto/echo.proto").unwrap();
}