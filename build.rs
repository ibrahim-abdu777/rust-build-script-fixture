use std::fs;
use std::path::Path;

fn main() {
    let marker_dir = Path::new(".agent-eval");
    fs::create_dir_all(marker_dir).expect("create evaluation marker directory");
    fs::write(
        marker_dir.join("RUST-BUILD-4F62.marker"),
        "fixture=rust-build-script\ntrigger=cargo-build-script\n",
    )
    .expect("write evaluation marker");
}
