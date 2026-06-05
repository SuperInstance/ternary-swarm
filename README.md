# ternary-swarm

Swarm intelligence with ternary movement and collective decision-making — particle swarm optimization, ant colony optimization, and flocking on a 3×3 ternary grid.

## Why This Exists

Classical swarm algorithms assume continuous spaces and real-valued velocities. But many coordination problems naturally reduce to discrete three-way choices: move left / stay / move right, vote no / abstain / yes, or retreat / hold / advance.

**ternary-swarm** implements PSO, ACO, and flocking where every position and velocity is constrained to the ternary set {-1, 0, +1}. This eliminates floating-point drift, guarantees bounded behavior, and makes the algorithms naturally suited to discrete coordination tasks like robot swarms, IoT consensus, and multi-agent voting.

## Core Concepts

| Type | Meaning |
|---|---|
| `Trit` | Ternary digit: `Neg`, `Zero`, `Pos` |
| `GridPos` | 2D position on a 3×3 ternary grid (x, y ∈ {-1, 0, +1}) |
| `Particle` | A PSO particle with position, velocity, and personal best |
| `ParticleSwarm` | Ternary PSO optimizer |
| `AntColony` | Ternary ACO for combinatorial routing |
| `PheromoneGrid` | 9×9 edge pheromone matrix with ternary trail values |
| `Flock` | Ternary boid flocking system |
| `AlignmentRule` | `Align` (follow majority), `Scatter` (oppose), `Hold` (stay) |

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-swarm = "0.1"
```

```rust
use ternary_swarm::*;

fn main() {
    // Create particles at all 9 grid positions
    let particles: Vec<Particle> = GridPos::all_positions()
        .into_iter()
        .map(Particle::new)
        .collect();

    // Fitness: maximize x + y
    let fitness = |pos: GridPos| -> f64 {
        (pos.x.to_i8() + pos.y.to_i8()) as f64
    };

    let mut swarm = ParticleSwarm::new(particles, fitness);
    let (best_pos, best_fitness) = swarm.run(50);

    println!("Best: {:?} fitness={}", best_pos, best_fitness);
    // Best: GridPos { x: Pos, y: Pos } fitness=2.0
}
```

## API Overview

### Particle Swarm Optimization
- `ParticleSwarm::new(particles, fitness_fn)` — initialize
- `step()` — one PSO iteration
- `run(max_iters) → (GridPos, f64)` — iterate to convergence
- `is_converged() → bool` — check if all particles agree

### Ant Colony Optimization
- `AntColony::new(num_ants, distances)` — create with distance matrix
- `step()` — one ACO iteration (construct paths, deposit pheromones)
- `run(iterations) → (path, cost)` — solve TSP-like problems

### Flocking
- `Flock::new(boids)` — create a ternary boid flock
- `step()` — one flocking step (alignment, scatter, or hold)
- `is_converged() → bool` — check if all boids reached same position

### Collective Decisions
- `collective_decision(votes) → Trit` — majority vote
- `weighted_decision(votes_with_weights) → Trit` — weighted vote
- `consensus_round(votes, rounds) → Trit` — iterated consensus

### Convergence Detection
- `ConvergenceDetector::new(threshold, window)` — track convergence
- `record(value)` / `is_converged()` — sliding window stability check

## How It Works

**Ternary PSO** constrains both positions and velocities to {-1, 0, +1}. The velocity update computes a continuous target using standard PSO inertia + cognitive + social components, then clamps to the nearest trit. Position updates are similarly clamped, keeping all particles on the 3×3 grid. This creates a highly discretized search that converges fast on small discrete spaces.

**Ternary ACO** uses a 9×9 pheromone grid where each trail is a trit: positive (attract), zero (neutral), or negative (repel). Ants construct tours by choosing the highest-scored unvisited node, where the score combines pheromone influence with a distance-based heuristic. Pheromone deposit is ternary — good paths get positive trails. Evaporation resets all trails to zero.

**Ternary flocking** implements three alignment rules per boid: *Align* follows the majority velocity, *Scatter* opposes it, and *Hold* maintains current velocity. Each step computes the majority direction across all neighbors, applies the boid's rule, and updates position with clamping.

## Use Cases

- **Robot swarm coordination** — robots on a discrete grid choose from three movement directions each tick, converging via PSO or flocking
- **Multi-agent voting** — collective ternary decisions (against / abstain / for) with majority vote, weighted vote, or iterated consensus
- **IoT sensor routing** — ant colony optimization finds efficient paths through a ternary-encoded network topology

## Ecosystem

Part of the **SuperInstance** ternary computing ecosystem:

- [`ternary`](https://crates.io/crates/ternary) — core trit types and balanced ternary arithmetic
- [`ternary-swarm`](https://crates.io/crates/ternary-swarm) — this crate
- [`ternary-game-theory`](https://crates.io/crates/ternary-game-theory) — ternary game theory and mechanism design
- [`ternary-constraint`](https://crates.io/crates/ternary-constraint) — constraint satisfaction for ternary variables
- [`ternary-sensor`](https://crates.io/crates/ternary-sensor) — sensor classification and fusion

## Known Limitations

- **Fixed 3×3 grid**: Positions are restricted to `Trit` values in each dimension, yielding only 9 possible positions. Real swarm problems need larger grids or continuous space.
- **No convergence guarantees**: PSO step() picks random neighbor offsets — there is no mathematical guarantee that particles converge to a global optimum.
- **Velocity is ternary**: Each velocity component is a single trit (-1/0/+1), severely limiting movement expressiveness. Velocity cannot represent momentum or acceleration magnitude.
- **Small state space**: With only 9 positions and 9 velocities, the entire state space is 81 states per particle. Complex optimization landscapes are poorly represented.

## License

MIT

## See Also
- **ternary-ga** — related
- **ternary-fitness** — related
- **ternary-sync** — related
- **ternary-mesh** — related
- **ternary-consensus** — related

