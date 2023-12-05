use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

// Data structure to represent the simulation state
#[derive(Clone)]
struct SimulationState {
    year: u32,
    population: u32,
}

// Function to perform the simulation step
async fn simulate_step(state: Arc<Mutex<SimulationState>>) {
    loop {
        sleep(Duration::from_secs(1)).await;

        let mut state = state.lock().unwrap();

        // Perform the simulation step
        state.year += 1;
        let births = rand::thread_rng().gen_range(1..=10);
        let deaths = rand::thread_rng().gen_range(0..=5);

        state.population = state.population.saturating_add(births).saturating_sub(deaths);

        println!(
            "Year: {}, Population: {}, Births: {}, Deaths: {}",
            state.year, state.population, births, deaths
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
    // Initial population between 50 and 100
    let initial_population = rand::thread_rng().gen_range(50..=100);

    // Create the simulation state
    let state = Arc::new(Mutex::new(SimulationState {
        year: 1,
        population: initial_population,
    }));

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
