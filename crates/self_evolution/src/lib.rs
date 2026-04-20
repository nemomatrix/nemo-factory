pub fn self_improve() {
    Runner::run_cycles(1000);

    let logs = std::fs::read_to_string("system.log").unwrap();

    let entries: Vec<_> = logs.lines()
        .filter_map(parse_line)
        .collect();

    let anomalies = Anomaly::detect(&entries);

    let clusters = cluster(anomalies);

    let weaknesses = detect_weakness(clusters, entries.len());

    for w in weaknesses {
        if Gate::allow(w.severity) {
            if let Some(fix) = Repair::propose(&w) {
                if validate_fix(&fix) {
                    println!("APPLY FIX: {}", fix);
                }
            }
        }
    }
}
