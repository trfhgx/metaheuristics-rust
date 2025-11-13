use std::collections::HashMap;
use rand::Rng;
use super::data_processer::{Data, CityData};
use crate::algos::Objective;
use crate::algos::local_search::Neighborhood;

pub type Solution = Vec<CityData>;

// this algorithm will
fn cycle_find(data:Data){

}

pub struct TSPObjective;
impl Objective<Solution, Data> for TSPObjective {
    // our objective function for this specfic TSP problem
    fn evaluate(&self, solution: &Solution) -> f64 {
        let mut eval: f64 = 0.0;
        for i in 0..solution.len() - 1 {
            let current = &solution[i];
            let next = &solution[i + 1];
            eval += ((current.x_km - next.x_km).powi(2)+ (current.y_km - next.y_km).powi(2)).sqrt();
        }
        if let (Some(last), Some(first)) = (solution.last(), solution.first()) {
            eval += ((last.x_km - first.x_km).powi(2) + (last.y_km - first.y_km).powi(2)).sqrt();
        }
        eval
    }

    fn random(&self, data: &Data) -> Solution {
        let mut rng = rand::rng();

        let algiers = data.get("Algiers").unwrap().clone();
        let mut solution = vec![algiers];

        let mut available_cities: Vec<String> = data.keys()
            .filter(|name| *name != "Algiers")
            .cloned()
            .collect();

        while !available_cities.is_empty() {
            let random_index = rng.random_range(0..available_cities.len());
            let city_name = available_cities.swap_remove(random_index);
            let city = data.get(&city_name).unwrap().clone();
            solution.push(city);
        }

        solution
    }

}

// TSP Neighborhood: 2-opt (reverse segments of the tour)
pub struct TSPNeighborhood;

impl Neighborhood<Solution> for TSPNeighborhood {
    fn neighbors(&self, solution: &Solution) -> Vec<Solution> {
        let mut neighbors = Vec::new();
        let n = solution.len();
        
        // Generate all 2-opt neighbors by reversing segments
        for i in 1..n-1 {
            for j in i+1..n {
                let mut neighbor = solution.clone();
                // Reverse the segment between i and j
               let mut left = i;
                let mut right = j;
                while left < right {
             neighbor.swap(left, right);
            left += 1;
             right -= 1;
                        }
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
}