mod utils;
mod algos;

use eframe::egui;
use utils::data_processer::{load_cities_data, CityData, Data};
use utils::tsp::{TSPObjective, TSPNeighborhood};
use algos::{RandomSearch, Objective};
use algos::local_search::{LocalSearch, hill_first_improv, hill_steepest_ascent};

fn main() -> eframe::Result {
    let data = load_cities_data("data/algeria_20_cities_xy.csv")
        .expect("Failed to load cities data");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Metaheuristics TSP",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(data)))),
    )
}

struct MyApp {
    iterations: String,
    result: Option<(f64, usize, String)>,
    solution: Option<Vec<CityData>>,
    algorithm: Algorithm,
    data: Data,
}

impl MyApp {
    fn new(data: Data) -> Self {
        Self {
            iterations: String::new(),
            result: None,
            solution: None,
            algorithm: Algorithm::default(),
            data,
        }
    }
}

#[derive(Default, PartialEq)]
enum Algorithm {
    #[default]
    RandomSearch,
    FirstImprovement,
    SteepestAscent,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("TSP Solver");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Iterations:");
                ui.text_edit_singleline(&mut self.iterations);
            });
            ui.add_space(10.0);

            if ui.button("Random Search").clicked() {
                if let Ok(iters) = self.iterations.parse::<usize>() {
                    self.algorithm = Algorithm::RandomSearch;
                    self.run_random_search(iters);
                }
            }
            if ui.button("Hill Climbing (First)").clicked() {
                self.algorithm = Algorithm::FirstImprovement;
                self.run_hill_climbing_first();
            }
            if ui.button("Hill Climbing (Steepest)").clicked() {
                self.algorithm = Algorithm::SteepestAscent;
                self.run_hill_climbing_steepest();
            }
            if ui.button("Simulated Annealing").clicked() {
            }
            if ui.button("Tabu Search").clicked() {
            }
            if ui.button("Genetic Algorithm").clicked() {
            }

            ui.add_space(20.0);
            if let Some((cost, iter_count, path)) = &self.result {
                ui.label(format!("Best Cost: {:.2}", cost));
                ui.label(format!("Iterations: {}", iter_count));
                ui.add_space(10.0);
                ui.label("Path:");
                ui.label(path);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(solution) = &self.solution {
                self.draw_tour(ui, solution);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Run an algorithm to see the tour visualization");
                });
            }
        });
    }
}

impl MyApp {
    fn draw_tour(&self, ui: &mut egui::Ui, solution: &[CityData]) {
        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), ui.available_height()),
            egui::Sense::hover()
        );
        
        if solution.is_empty() {
            return;
        }
        
        let min_x = solution.iter().map(|c| c.x_km).fold(f64::INFINITY, f64::min);
        let max_x = solution.iter().map(|c| c.x_km).fold(f64::NEG_INFINITY, f64::max);
        let min_y = solution.iter().map(|c| c.y_km).fold(f64::INFINITY, f64::min);
        let max_y = solution.iter().map(|c| c.y_km).fold(f64::NEG_INFINITY, f64::max);
        
        let margin = 30.0;
        let width = rect.width() - 2.0 * margin;
        let height = rect.height() - 2.0 * margin;
        
        let scale_x = width / (max_x - min_x) as f32;
        let scale_y = height / (max_y - min_y) as f32;
        
        let to_screen = |city: &CityData| -> egui::Pos2 {
            let x = margin + ((city.x_km - min_x) as f32) * scale_x;
            let y = margin + ((max_y - city.y_km) as f32) * scale_y;
            rect.min + egui::vec2(x, y)
        };
        
        let painter = ui.painter();
        
        for i in 0..solution.len() {
            let current = &solution[i];
            let next = &solution[(i + 1) % solution.len()];
            
            let start = to_screen(current);
            let end = to_screen(next);
            
            painter.line_segment(
                [start, end],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 255))
            );
            
            let dir = (end - start).normalized();
            let arrow_size = 8.0;
            let arrow_pos = start + dir * ((end - start).length() * 0.5);
            let perp = egui::vec2(-dir.y, dir.x);
            
            painter.line_segment(
                [arrow_pos, arrow_pos - dir * arrow_size + perp * arrow_size * 0.5],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 255))
            );
            painter.line_segment(
                [arrow_pos, arrow_pos - dir * arrow_size - perp * arrow_size * 0.5],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 255))
            );
        }
        
        for city in solution {
            let pos = to_screen(city);
            painter.circle_filled(pos, 5.0, egui::Color32::from_rgb(255, 100, 100));
            painter.text(
                pos + egui::vec2(8.0, -8.0),
                egui::Align2::LEFT_BOTTOM,
                &city.city_name,
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE
            );
        }
    }

    fn run_random_search(&mut self, iterations: usize) {
        let objective = TSPObjective;
        let mut search = RandomSearch::new(objective, &self.data);
        let (solution, cost) = search.run(&self.data, iterations);
        
        let path = solution.iter()
            .map(|city| city.city_name.as_str())
            .collect::<Vec<_>>()
            .join(" -> ");
        
        self.solution = Some(solution);
        self.result = Some((cost, iterations, path));
    }

    fn run_hill_climbing_first(&mut self) {
        let objective = TSPObjective;
        let neighborhood = TSPNeighborhood;
        let mut search = LocalSearch::new(objective, neighborhood);
        
        let initial = TSPObjective.random(&self.data);
        search.set_initial(initial);
        
        let (solution, cost, iterations) = hill_first_improv::run(&mut search);
        
        let path = solution.iter()
            .map(|city| city.city_name.as_str())
            .collect::<Vec<_>>()
            .join(" -> ");
        
        self.solution = Some(solution);
        self.result = Some((cost, iterations, path));
    }

    fn run_hill_climbing_steepest(&mut self) {
        let objective = TSPObjective;
        let neighborhood = TSPNeighborhood;
        let mut search = LocalSearch::new(objective, neighborhood);
        
        let initial = TSPObjective.random(&self.data);
        search.set_initial(initial);
        
        let (solution, cost, iterations) = hill_steepest_ascent::run(&mut search);
        
        let path = solution.iter()
            .map(|city| city.city_name.as_str())
            .collect::<Vec<_>>()
            .join(" -> ");
        
        self.solution = Some(solution);
        self.result = Some((cost, iterations, path));
    }
}
