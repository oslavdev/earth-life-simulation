# Earth Life Simulation

## Overview

This project implements a simplified simulation of life on Earth using Rust. The simulation considers basic demographic factors such as birth and death rates and runs over a specified period.

## Features

- **Population Dynamics:** The simulation models the growth and decline of a population over time.
  
- **Randomized Initial Conditions:** The initial population is randomly generated, considering a range of values for the number of people, genders, and age distribution.

- **Life Events:** Individuals in the simulation experience life events such as birth, aging, and death.

## How It Works

The simulation progresses year by year, with each iteration representing a single year in the simulation. It provides real-time updates on the current year, total population, births, and deaths.

## Getting Started

### Prerequisites

- **Rust:** Ensure that you have Rust installed on your machine. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

- **Docker (Optional):** If you prefer running the simulation in a Docker container, make sure Docker is installed on your system.

### Build and Run Locally

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/earth-life-simulation.git
   cd earth-life-simulation
``````

2. **Start application: **

```bash
    cargo run
```

Access the simulation updates at http://localhost:8080 in your web browser.

### Customize the Simulation

Feel free to explore and modify the simulation parameters in the source code to observe different population dynamics.