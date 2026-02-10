use polars::prelude::*;

pub fn add(a: i16, b: i16) -> i16 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let c = add(1, 2);
        assert_eq!(c, 3, "addition doesn not work on {} {}", 1, 2);
    }
}

fn main() {
    println!("{} + {}: {:.2}", 2, 3,  add(2, 3));
    let my_df = df![
        "names" => ["Daniel", "Sveta", "Igor"],
        "money" => [12, 16, -11],
    ].unwrap();

    println!("{}", my_df.head(Some(2)));
}
