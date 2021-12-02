#[macro_export]
macro_rules! aoc {
    ($( $x:ident ),*) => {
        $(pub mod $x;)*
            pub fn build_solutions() -> std::collections::BTreeMap<String, Solution> {
                let mut solutions = std::collections::BTreeMap::new();
                $(solutions.insert(stringify!($x).to_owned(), Solution{
                    part1: $x::part1,
                    part2: $x::part2
                });)*
                solutions
            }
    };
}
