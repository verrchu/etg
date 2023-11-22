#[derive(Debug, PartialEq, Eq, etg_derive::Tags)]
enum E {
    #[serde(rename = "test_a")]
    A,
    #[serde(rename = "test_b")]
    B,
}

fn main() {
    assert_eq!(E::tags(), &["test_a", "test_b"]);

    assert_eq!(E::by_tag("test_a"), Some(E::A));
    assert_eq!(E::by_tag("test_b"), Some(E::B));
    assert_eq!(E::by_tag("test_c"), None);

    assert_eq!(E::A.tag(), "test_a");
    assert_eq!(E::B.tag(), "test_b");
}
