#[derive(Debug, Clone)]
pub struct RoundDoubles {
    pub round_number: usize,
    pub matches: Vec<DoubleMatch>,
}

#[derive(Debug, Clone)]
pub struct RoundSingles {
    pub round_number: usize,
    pub matches: Vec<SingleMatch>,
}

#[derive(Debug, Clone)]
pub struct DoubleMatch {
    pub double_a: (usize, usize),
    pub double_b: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct SingleMatch {
    pub a: usize,
    pub b: usize,
}