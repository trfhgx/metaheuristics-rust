pub trait Neighborhood<S> {
    fn neighbors(&self, solution: &S) -> Vec<S>;
}

pub struct LocalSearch<S, O, N>
where
    S: Clone,
    N: Neighborhood<S>,
{
    pub objective: O,
    pub neighborhood: N,
    pub current_solution: Option<S>,
}

impl<S, O, N> LocalSearch<S, O, N>
where
    S: Clone,
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

