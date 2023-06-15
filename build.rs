fn main() {
    let proto_file = "./proto/voter.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./proto")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}
