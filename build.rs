fn main() {
    println!(
        "cargo:rerun-if-changed={}",
        commitment_issues::find_valid_git_root!()
    );
}
