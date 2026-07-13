use rgrep::search;

#[test]
fn returns_only_matching_lines() {
    let contents = "Rust:\nsafe, fast, productive.\nPick three.";

    assert_eq!(search("duct", contents), vec!["safe, fast, productive."]);
}

#[test]
fn returns_no_lines_when_query_is_absent() {
    assert!(search("monomorphization", "Rust:\nsafe and fast.").is_empty());
}

#[test]
fn returns_all_matches_in_input_order() {
    let contents = "safe, fast, productive.\nPick three.\nRust is fast.";

    assert_eq!(
        search("fast", contents),
        vec!["safe, fast, productive.", "Rust is fast."]
    );
}

#[test]
fn search_is_case_sensitive() {
    let contents = "Rust\nrust\nRUST";

    assert_eq!(search("Rust", contents), vec!["Rust"]);
}

#[test]
fn empty_query_matches_each_line() {
    let contents = "first\nsecond";

    assert_eq!(search("", contents), vec!["first", "second"]);
}
