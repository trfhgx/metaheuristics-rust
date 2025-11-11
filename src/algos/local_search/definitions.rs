

pub trait Objective<S> {
    fn evaluate(&self, solution: &S) -> f64;
}

pub trait Neighborhood<S> {
    fn neighbors(&self, solution: &S) -> Vec<S>;
}
pub struct LocalSearch<S, O, N>
where
    S: Clone,
    O: Objective<S>,
    N: Neighborhood<S>,
{
    objective: O,
    neighborhood: N,
    current_solution: Option<S>,
}


impl<S, O, N> LocalSearch<S, O, N>
where
    S: Clone,
    O: Objective<S>,
    N: Neighborhood<S>,
{
    pub fn new(objective: O, neighborhood: N) -> Self {
        Self {
            objective,
            neighborhood,
            current_solution: None,
        }
    }

    pub fn set_initial(&mut self, initial: S) {
        self.current_solution = Some(initial);
    }
}