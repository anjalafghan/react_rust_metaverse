fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile_protos(&["../proto/helloworld.proto"], &["../proto"])
        .unwrap();

    println!("cargo:rerun-if-changed=../proto/helloworld.proto");
}
