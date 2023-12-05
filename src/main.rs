use actix_web::{web, App, HttpServer, Responder};
use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

// Data structure to represent the simulation state
#[derive(Clone)]
struct SimulationState {
    year: u32,
    population: u32,
    age_distribution: AgeDistribution,
}

// Data structure to represent age groups
#[derive(Debug, Clone)]
struct AgeDistribution {
    age_groups: Vec<AgeGroup>,
}

// Data structure for each age group
#[derive(Debug, Clone)]
struct AgeGroup {
    min_age: u32,
    max_age: u32,
    birth_rate: f64,
    death_rate: f64,
}

impl SimulationState {
    fn new() -> Self {
        // Initialize age groups with birth and death rates
        let age_distribution = AgeDistribution {
            age_groups: vec![
                AgeGroup {
                    min_age: 16,
                    max_age: 21,
                    birth_rate: 0.2,
                    death_rate: 0.1,
                },
                AgeGroup {
                    min_age: 21,
                    max_age: 30,
                    birth_rate: 0.3,
                    death_rate: 0.05,
                },
                AgeGroup {
                    min_age: 31,
                    max_age: 40,
                    birth_rate: 0.25,
                    death_rate: 0.02,
                },
                AgeGroup {
                    min_age: 41,
                    max_age: 60,
                    birth_rate: 0.25,
                    death_rate: 0.02,
                },
            ],
        };

        SimulationState {
            year: 1,
            population: 100,
            age_distribution,
        }
    }

    fn get_birth_rate(&self, age: u32) -> f64 {
        // Find the birth rate for the corresponding age group
        self.age_distribution
            .age_groups
            .iter()
            .find(|&group| age >= group.min_age && age <= group.max_age)
            .map_or(0.0, |group| group.birth_rate)
    }

    fn get_death_rate(&self, age: u32) -> f64 {
        // Find the death rate for the corresponding age group
        self.age_distribution
            .age_groups
            .iter()
            .find(|&group| age >= group.min_age && age <= group.max_age)
            .map_or(0.0, |group| group.death_rate)
    }
}

async fn simulate_step(state: Arc<Mutex<SimulationState>>) {
    loop {
        sleep(Duration::from_secs(1)).await;

        let mut state = state.lock().unwrap();

        // Perform the simulation step
        state.year += 1;

        // Simulate events like famine or diseases affecting birth and death rates
        let famine_event = rand::thread_rng().gen_bool(0.05); // 5% chance of famine
        let disease_event = rand::thread_rng().gen_bool(0.2); // 20% chance of disease

        for age_group in state.age_distribution.age_groups.iter_mut() {
            // Adjust birth rates based on events
            if famine_event {
                age_group.birth_rate *= 0.5; // Reduce birth rate by 50% during famine
            }

            // Adjust death rates based on events
            if disease_event {
                age_group.death_rate *= 2.0; // Double death rate during a disease outbreak
            }
        }

        let total_births: u32 = state
            .age_distribution
            .age_groups
            .iter()
            .map(|group| (state.population as f64 * group.birth_rate).round() as u32)
            .sum();

        let total_deaths: u32 = state
            .age_distribution
            .age_groups
            .iter()
            .map(|group| (state.population as f64 * group.death_rate).round() as u32)
            .sum();

        state.population = state.population.saturating_add(total_births).saturating_sub(total_deaths);

        let famine_info = if famine_event { println!("(Famine!)") } else { println!("") };
        let disease_info = if disease_event { println!("(Disease!)") } else { println!("") };

        println!(
            "Year: {}, Population: {}, Total Births: {}, Total Deaths: {}",
            state.year, state.population, total_births, total_deaths
        );
    }
}

// Handler for the web endpoint
async fn index(state: web::Data<Arc<Mutex<SimulationState>>>) -> impl Responder {
    let state = state.lock().unwrap();
    format!(
        "Year: {}, Population: {}",
        state.year, state.population
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the simulation state
    let state = Arc::new(Mutex::new(SimulationState::new()));

    // Spawn the simulation loop as a separate task
    tokio::spawn(simulate_step(state.clone()));

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
