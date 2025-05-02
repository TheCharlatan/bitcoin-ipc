use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(
        env::var("OUT_DIR").expect("OUT_DIR was not defined by the cargo environment!"),
    );
    println!("cargo:rerun-if-changed=capnp");
    capnpc::CompilerCommand::new()
        .src_prefix("capnp")
        .file("capnp/common.capnp")
        .file("capnp/echo.capnp")
        .file("capnp/init.capnp")
        .file("capnp/mining.capnp")
        .file("capnp/proxy.capnp")
        .output_path(out_path)
        .run()
        .unwrap();
}
