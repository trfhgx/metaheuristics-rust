// pretty much our interface
pub trait Objective<S, Set>{
    fn evaluate(&self, solution: &S) -> f64;
    fn random(&self, data: &Set) -> S;
}

pub mod local_search;
pub mod random_search;

pub use random_search::RandomSearch;
