#[derive(etg_derive::Tags)]
enum E {
    #[serde(rename = "a")]
    A,
    B,
    #[serde(rename = "c")]
    C,
}

fn main() {}
