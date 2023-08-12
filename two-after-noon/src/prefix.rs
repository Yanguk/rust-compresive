// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // request_path 가 prefix보다 길때 의 케이스
    // for (prefix, path) in prefix.split('/').zip(request_path.split('/')) {
    //     println!("prefix: {prefix} // path: {path}");
    //
    //     let is_match = prefix == path || prefix == "*";
    //
    //     if !is_match {
    //         return false
    //     }
    // }

    // true

    let mut request_segments = request_path.split('/');

    for prefix_segment in prefix.split('/') {
        let Some(request_segment) = request_segments.next() else {
            return false;
        };

        if prefix_segment != request_segment && prefix_segment != "*" {
            return false;
        }
    }

    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

