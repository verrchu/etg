#[derive(etg_derive::Tags)]
enum E {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B(u32),
}

fn main() {}
