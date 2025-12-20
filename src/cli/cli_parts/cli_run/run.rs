pub fn run() {
    let matches = build_matches();

    if try_run_standard_json(&matches) {
        return;
    }

    run_single_file(&matches);
}

