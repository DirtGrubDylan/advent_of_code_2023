pub trait Location<RHS = Self> {
    type ValueOutput;

    fn manhattan_distance_to(&self, other: &RHS) -> Self::ValueOutput;

    fn distance_to(&self, other: &RHS) -> f64;

    #[must_use]
    fn add(&self, other: &RHS) -> Self;
}
