use crate::algos::Objective;

// the main struct 
pub struct RandomSearch<S, O>
where S: Clone
{
    current_solution: S,
    objective: O,
    used_space: Vec<S>
}

// ------ Implementation starts here ----
impl<S, O> RandomSearch<S, O>
where
    S: Clone + PartialEq
{
    // init
    pub fn new<Set>(objective: O, data: &Set) -> Self 
    where
        O: Objective<S, Set>
    {
        let initial_solution = objective.random(data);
        Self {
            current_solution: initial_solution.clone(),
            objective,
            used_space: vec![initial_solution]
        }
    }
// this makes sure we dont revisit solutions
    pub fn generate_unique_solution<Set>(&mut self, data: &Set) -> S 
    where
        O: Objective<S, Set>
    {
        loop {
            let candidate = self.objective.random(data);

            if !self.used_space.contains(&candidate) {
                self.used_space.push(candidate.clone());
                self.current_solution = candidate.clone();
                return candidate;
            }
        }
    }
// this is the main run function for number of iterations
    pub fn run<Set>(&mut self, data: &Set, iterations: usize) -> (S, f64) 
    where
        O: Objective<S, Set>
    {
        let mut best_solution = self.generate_unique_solution(data);
        let mut best_cost = self.objective.evaluate(&best_solution);

        for _ in 1..iterations {
            let new_solution = self.generate_unique_solution(data);
            let new_cost = self.objective.evaluate(&new_solution);

            if new_cost < best_cost {
                best_cost = new_cost;
                best_solution = new_solution;
            }
        }
        (best_solution, best_cost)
    }
}
