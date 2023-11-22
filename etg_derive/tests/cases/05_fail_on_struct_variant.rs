#[derive(etg_derive::Tags)]
enum E {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B { x: u32 },
}

fn main() {}
