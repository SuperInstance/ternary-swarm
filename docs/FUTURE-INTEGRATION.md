# Future Integration: ternary-swarm

## Current State
Provides swarm intelligence with ternary movement and decision making: particles on a ternary grid, particle swarm optimization, ant colony optimization with ternary pheromones, and flocking behaviors.

## Integration Opportunities

### With ternary-cell (Population Dynamics)
Cells in a grid are a swarm. Each cell is a particle with position (its grid coordinates) and velocity (its state transition tendency). PSO on the cell grid optimizes cell positions for maximum collective intelligence. Ant colony pheromones map to `TernaryMessenger` accumulation — cells that receive many messages build up "pheromone trails" that guide future messages.

### With ternary-robotics
Physical robots in a room ARE a swarm. `GridPos` with ternary coordinates discretizes robot positions. Flocking rules (separation, alignment, cohesion) on ternary vectors give simple but robust multi-robot coordination. `TernaryActuator` commands from `ternary-robotics` become the movement primitives that `ternary-swarm` orchestrates.

### With ternary-network
Swarm topology IS a network. As particles move, they form and dissolve connections. `TernaryNetwork` tracks the dynamic connectivity. PSO neighborhood structures (ring, star, von Neumann) map to network topologies.

## Potential in Mature Systems
In room-as-codespace, agents are a swarm navigating the campus. Each Codespace is a food source; agents leave digital pheromones (tiles in PLATO) indicating which rooms are productive. New agents follow pheromone trails to the best rooms. Flocking ensures agents don't all cluster in one room — they spread out to cover the campus.

## Cross-Pollination Ideas
- PSO for room parameter optimization — find the best room configuration through swarm search
- Ant colony for path planning — optimal agent routes through room campuses
- Pheromone decay as room relevance scoring — old pheromones decay, keeping recommendations fresh

## Dependencies for Next Steps
- ternary-cell needs swarm-based GC strategy (pheromone-guided pruning)
- ternary-robotics needs swarm coordination layer
- Integration with ternary-network for dynamic topology management
