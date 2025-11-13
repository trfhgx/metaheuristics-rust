use crate::algos::Objective;
use super::definitions::{LocalSearch, Neighborhood};

pub fn run<S, O, N, Set>(
    search: &mut LocalSearch<S, O, N>,
) -> (S, f64, usize)
where
    S: Clone,
    O: Objective<S, Set>,
    N: Neighborhood<S>,
{
    let mut current = search.current_solution.clone().expect("No initial solution set");
    let mut current_cost = search.objective.evaluate(&current);
    let mut iterations = 0;
    
    loop {
        let neighbors = search.neighborhood.neighbors(&current);
        let mut improved = false;
        
        for neighbor in neighbors {
            let neighbor_cost = search.objective.evaluate(&neighbor);
            if neighbor_cost < current_cost {
                current = neighbor;
                current_cost = neighbor_cost;
                improved = true;
                break;
            }
        }
        
        iterations += 1;
        
        if !improved {
            break;
        }
    }
    
    (current, current_cost, iterations)
}
