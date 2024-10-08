use run_make_support::{python_command, rustdoc};

fn main() {
    let out_dir = "out";
    rustdoc()
        .input("foo.rs")
        .arg("-Zunstable-options")
        .arg("--generate-redirect-map")
        .out_dir(&out_dir)
        .run();
    // FIXME (GuillaumeGomez): Port the python script to Rust as well.
    python_command().arg("validate_json.py").arg(&out_dir).run();
}
