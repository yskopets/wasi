use std::env;
use std::path::PathBuf;

fn main() {
    // let witx_path = wasi_dir.join("phases/snapshot/witx/wasi_snapshot_preview1.witx");
    let witx_path: PathBuf = env::args_os().nth(1).unwrap().into();
    print!("{}", generate_raw::generate(&witx_path));
}
