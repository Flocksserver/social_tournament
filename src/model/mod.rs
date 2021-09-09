/// Struct for a double tournament that represent one round. It holds the `round_number`
/// and the `matches` that take place in this round. Matches are a list of [DoubleMatch].
#[derive(Debug, Clone)]
pub struct RoundDoubles {
    pub round_number: usize,
    pub matches: Vec<DoubleMatch>,
}
/// Struct for a single tournament that represent one round. It holds the `round_number`
/// and the `matches` that take place in this round. Matches are a list of [SingleMatch].
#[derive(Debug, Clone)]
pub struct RoundSingles {
    pub round_number: usize,
    pub matches: Vec<SingleMatch>,
}
/// Struct that represent a double match. It holds the pairs `double_a` and `double_b`.
/// The [Tuple] contains two numbers, the unique player ids.
#[derive(Debug, Clone)]
pub struct DoubleMatch {
    pub double_a: (usize, usize),
    pub double_b: (usize, usize),
}
/// Struct that represent a single match. It holds the opponents `a` and `b`.
/// The unique numbers is the id of the corresponding player.
#[derive(Debug, Clone)]
pub struct SingleMatch {
    pub a: usize,
    pub b: usize,
}