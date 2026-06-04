#![forbid(unsafe_code)]

//! Swarm intelligence with ternary movement and decision making.

/// A ternary value: Negative (-1), Zero (0), or Positive (+1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Trit {
    Neg,
    Zero,
    Pos,
}

impl Trit {
    pub fn to_i8(self) -> i8 {
        match self {
            Trit::Neg => -1,
            Trit::Zero => 0,
            Trit::Pos => 1,
        }
    }

    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Trit::Neg),
            0 => Some(Trit::Zero),
            1 => Some(Trit::Pos),
            _ => None,
        }
    }

    /// Balanced ternary NOT (swap pos/neg).
    pub fn ternary_not(self) -> Self {
        match self {
            Trit::Neg => Trit::Pos,
            Trit::Zero => Trit::Zero,
            Trit::Pos => Trit::Neg,
        }
    }
}

/// A 2D position on a ternary grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPos {
    pub x: Trit,
    pub y: Trit,
}

impl GridPos {
    pub fn new(x: Trit, y: Trit) -> Self {
        Self { x, y }
    }

    /// All 9 positions on a 3x3 ternary grid.
    pub fn all_positions() -> Vec<GridPos> {
        let trits = [Trit::Neg, Trit::Zero, Trit::Pos];
        let mut positions = Vec::new();
        for &x in &trits {
            for &y in &trits {
                positions.push(GridPos::new(x, y));
            }
        }
        positions
    }

    pub fn manhattan_distance(self, other: GridPos) -> i8 {
        let dx = self.x.to_i8() - other.x.to_i8();
        let dy = self.y.to_i8() - other.y.to_i8();
        dx.abs() + dy.abs()
    }
}

/// A particle in a ternary particle swarm.
#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: GridPos,
    pub velocity: GridPos,
    pub best_pos: GridPos,
    pub best_fitness: f64,
}

impl Particle {
    pub fn new(pos: GridPos) -> Self {
        Self {
            pos,
            velocity: GridPos::new(Trit::Zero, Trit::Zero),
            best_pos: pos,
            best_fitness: f64::NEG_INFINITY,
        }
    }
}

/// Fitness function type.
pub type FitnessFn = fn(GridPos) -> f64;

/// Ternary Particle Swarm Optimization.
pub struct ParticleSwarm {
    pub particles: Vec<Particle>,
    pub global_best_pos: GridPos,
    pub global_best_fitness: f64,
    fitness: FitnessFn,
    inertia: f64,
    cognitive: f64,
    social: f64,
}

impl ParticleSwarm {
    pub fn new(particles: Vec<Particle>, fitness: FitnessFn) -> Self {
        let mut swarm = Self {
            particles,
            global_best_pos: GridPos::new(Trit::Zero, Trit::Zero),
            global_best_fitness: f64::NEG_INFINITY,
            fitness,
            inertia: 0.5,
            cognitive: 1.0,
            social: 1.5,
        };
        swarm.evaluate_all();
        swarm
    }

    fn evaluate_all(&mut self) {
        for p in &mut self.particles {
            let fit = (self.fitness)(p.pos);
            p.best_fitness = fit;
            p.best_pos = p.pos;
            if fit > self.global_best_fitness {
                self.global_best_fitness = fit;
                self.global_best_pos = p.pos;
            }
        }
    }

    /// Advance one iteration.
    pub fn step(&mut self) {
        for p in &mut self.particles {
            // Ternary velocity update: move toward global best
            let dx = self.global_best_pos.x.to_i8() - p.pos.x.to_i8();
            let dy = self.global_best_pos.y.to_i8() - p.pos.y.to_i8();
            let vx = clamp_trit((self.inertia * p.velocity.x.to_i8() as f64 + self.cognitive * (p.best_pos.x.to_i8() - p.pos.x.to_i8()) as f64 + self.social * dx as f64) as i8);
            let vy = clamp_trit((self.inertia * p.velocity.y.to_i8() as f64 + self.cognitive * (p.best_pos.y.to_i8() - p.pos.y.to_i8()) as f64 + self.social * dy as f64) as i8);
            p.velocity = GridPos::new(Trit::from_i8(vx).unwrap_or(Trit::Zero), Trit::from_i8(vy).unwrap_or(Trit::Zero));

            // Update position
            let nx = clamp_trit(p.pos.x.to_i8() + p.velocity.x.to_i8());
            let ny = clamp_trit(p.pos.y.to_i8() + p.velocity.y.to_i8());
            if let (Some(xt), Some(yt)) = (Trit::from_i8(nx), Trit::from_i8(ny)) {
                p.pos = GridPos::new(xt, yt);
            }

            let fit = (self.fitness)(p.pos);
            if fit > p.best_fitness {
                p.best_fitness = fit;
                p.best_pos = p.pos;
            }
            if fit > self.global_best_fitness {
                self.global_best_fitness = fit;
                self.global_best_pos = p.pos;
            }
        }
    }

    /// Run until converged or max iterations.
    pub fn run(&mut self, max_iters: usize) -> (GridPos, f64) {
        for _ in 0..max_iters {
            let prev = self.global_best_fitness;
            self.step();
            if (self.global_best_fitness - prev).abs() < 1e-10 {
                break;
            }
        }
        (self.global_best_pos, self.global_best_fitness)
    }

    pub fn is_converged(&self) -> bool {
        self.particles.iter().all(|p| p.pos == self.global_best_pos)
    }
}

fn clamp_trit(v: i8) -> i8 {
    v.max(-1).min(1)
}

/// Pheromone trails on a ternary grid: +1 attract, 0 neutral, -1 repel.
#[derive(Clone, Debug)]
pub struct PheromoneGrid {
    /// (9x9) edge pheromone values between grid positions.
    trails: Vec<Vec<Trit>>,
    size: usize,
}

impl PheromoneGrid {
    pub fn new() -> Self {
        let size = 9; // 9 positions in ternary grid
        Self {
            trails: vec![vec![Trit::Zero; size]; size],
            size,
        }
    }

    pub fn set_trail(&mut self, from: usize, to: usize, pheromone: Trit) {
        if from < self.size && to < self.size {
            self.trails[from][to] = pheromone;
        }
    }

    pub fn get_trail(&self, from: usize, to: usize) -> Trit {
        if from < self.size && to < self.size {
            self.trails[from][to]
        } else {
            Trit::Zero
        }
    }

    /// Evaporate all trails toward Zero.
    pub fn evaporate(&mut self) {
        // In ternary, evaporation means all trails become Zero
        for row in &mut self.trails {
            for cell in row.iter_mut() {
                *cell = Trit::Zero;
            }
        }
    }

    /// Deposit pheromone along a path.
    pub fn deposit(&mut self, path: &[usize], pheromone: Trit) {
        for window in path.windows(2) {
            self.trails[window[0]][window[1]] = pheromone;
        }
    }
}

/// An ant in the ternary ant colony.
#[derive(Clone, Debug)]
pub struct Ant {
    pub position: usize,
    pub path: Vec<usize>,
    pub visited: Vec<bool>,
    pub total_cost: f64,
}

impl Ant {
    pub fn new(start: usize, grid_size: usize) -> Self {
        let mut visited = vec![false; grid_size];
        visited[start] = true;
        Self {
            position: start,
            path: vec![start],
            visited,
            total_cost: 0.0,
        }
    }

    pub fn visit(&mut self, next: usize, cost: f64) {
        self.visited[next] = true;
        self.position = next;
        self.path.push(next);
        self.total_cost += cost;
    }

    pub fn has_visited(&self, pos: usize) -> bool {
        self.visited.get(pos).copied().unwrap_or(false)
    }

    pub fn is_complete(&self) -> bool {
        self.visited.iter().all(|v| *v)
    }
}

/// Ternary Ant Colony Optimization.
pub struct AntColony {
    pub ants: Vec<Ant>,
    pub pheromones: PheromoneGrid,
    pub distances: Vec<Vec<f64>>,
    pub best_path: Vec<usize>,
    pub best_cost: f64,
    alpha: f64,
    beta: f64,
}

impl AntColony {
    pub fn new(num_ants: usize, distances: Vec<Vec<f64>>) -> Self {
        let n = distances.len();
        let ants = (0..num_ants).map(|i| Ant::new(i % n, n)).collect();
        Self {
            ants,
            pheromones: PheromoneGrid::new(),
            distances,
            best_path: Vec::new(),
            best_cost: f64::INFINITY,
            alpha: 1.0,
            beta: 2.0,
        }
    }

    /// Choose next position for an ant based on ternary pheromone influence.
    pub fn choose_next(&self, ant: &Ant) -> Option<usize> {
        let n = self.distances.len();
        let mut candidates: Vec<(usize, f64)> = Vec::new();
        for j in 0..n {
            if !ant.has_visited(j) && self.distances[ant.position][j] > 0.0 {
                let pheromone = self.pheromones.get_trail(ant.position, j).to_i8() as f64;
                let attraction = (1.0 + pheromone).max(0.1).powf(self.alpha);
                let heuristic = (1.0 / self.distances[ant.position][j]).powf(self.beta);
                candidates.push((j, attraction * heuristic));
            }
        }
        if candidates.is_empty() {
            return None;
        }
        // Simple: pick highest scored
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Some(candidates[0].0)
    }

    /// Run one iteration of ant colony.
    pub fn step(&mut self) {
        let n = self.distances.len();
        let distances = self.distances.clone();
        let mut new_best_path = self.best_path.clone();
        let mut new_best_cost = self.best_cost;

        for ant in &mut self.ants {
            loop {
                let pos = ant.position;
                let visited = ant.visited.clone();
                let mut candidates: Vec<(usize, f64)> = Vec::new();
                for j in 0..n {
                    if !visited[j] && distances[pos][j] > 0.0 {
                        let pheromone = self.pheromones.get_trail(pos, j).to_i8() as f64;
                        let attraction = (1.0 + pheromone).max(0.1).powf(self.alpha);
                        let heuristic = (1.0 / distances[pos][j]).powf(self.beta);
                        candidates.push((j, attraction * heuristic));
                    }
                }
                if candidates.is_empty() {
                    break;
                }
                candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                let next = candidates[0].0;
                let cost = distances[pos][next];
                ant.visit(next, cost);
            }
            if ant.is_complete() && ant.total_cost < new_best_cost {
                new_best_cost = ant.total_cost;
                new_best_path = ant.path.clone();
            }
        }
        self.best_cost = new_best_cost;
        self.best_path = new_best_path;

        // Deposit pheromones on best path
        if !self.best_path.is_empty() {
            self.pheromones.deposit(&self.best_path, Trit::Pos);
        }
        // Reset ants
        for (i, ant) in self.ants.iter_mut().enumerate() {
            *ant = Ant::new(i % n, n);
        }
    }

    pub fn run(&mut self, iterations: usize) -> (&[usize], f64) {
        for _ in 0..iterations {
            self.step();
        }
        (&self.best_path, self.best_cost)
    }
}

/// Ternary alignment rule for flocking.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignmentRule {
    /// Align with majority.
    Align,
    /// Oppose majority (scatter).
    Scatter,
    /// Stay put.
    Hold,
}

impl AlignmentRule {
    pub fn from_trit(t: Trit) -> Self {
        match t {
            Trit::Pos => AlignmentRule::Align,
            Trit::Neg => AlignmentRule::Scatter,
            Trit::Zero => AlignmentRule::Hold,
        }
    }

    pub fn to_trit(self) -> Trit {
        match self {
            AlignmentRule::Align => Trit::Pos,
            AlignmentRule::Scatter => Trit::Neg,
            AlignmentRule::Hold => Trit::Zero,
        }
    }
}

/// A boid (bird-oid) for ternary flocking.
#[derive(Clone, Debug)]
pub struct Boid {
    pub pos: GridPos,
    pub vel: GridPos,
    pub rule: AlignmentRule,
}

impl Boid {
    pub fn new(pos: GridPos, rule: AlignmentRule) -> Self {
        Self {
            pos,
            vel: GridPos::new(Trit::Zero, Trit::Zero),
            rule,
        }
    }
}

/// Ternary flocking system.
pub struct Flock {
    pub boids: Vec<Boid>,
}

impl Flock {
    pub fn new(boids: Vec<Boid>) -> Self {
        Self { boids }
    }

    /// Compute majority direction of neighbors.
    pub fn majority_direction(&self, index: usize) -> (Trit, Trit) {
        let boid = &self.boids[index];
        let mut sx = 0i8;
        let mut sy = 0i8;
        for (i, other) in self.boids.iter().enumerate() {
            if i != index {
                sx += other.vel.x.to_i8();
                sy += other.vel.y.to_i8();
            }
        }
        (
            Trit::from_i8(sx.signum()).unwrap_or(Trit::Zero),
            Trit::from_i8(sy.signum()).unwrap_or(Trit::Zero),
        )
    }

    /// Update one step of flocking.
    pub fn step(&mut self) {
        let new_vels: Vec<GridPos> = self.boids.iter().enumerate().map(|(i, boid)| {
            let (mx, my) = self.majority_direction(i);
            match boid.rule {
                AlignmentRule::Align => GridPos::new(mx, my),
                AlignmentRule::Scatter => GridPos::new(mx.ternary_not(), my.ternary_not()),
                AlignmentRule::Hold => boid.vel,
            }
        }).collect();

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.vel = new_vels[i];
            let nx = clamp_trit(boid.pos.x.to_i8() + boid.vel.x.to_i8());
            let ny = clamp_trit(boid.pos.y.to_i8() + boid.vel.y.to_i8());
            if let (Some(xt), Some(yt)) = (Trit::from_i8(nx), Trit::from_i8(ny)) {
                boid.pos = GridPos::new(xt, yt);
            }
        }
    }

    /// Check if flock has converged (all same position).
    pub fn is_converged(&self) -> bool {
        if self.boids.is_empty() {
            return true;
        }
        let first = self.boids[0].pos;
        self.boids.iter().all(|b| b.pos == first)
    }
}

/// Swarm convergence detector.
pub struct ConvergenceDetector {
    pub history: Vec<f64>,
    pub threshold: f64,
    pub window: usize,
}

impl ConvergenceDetector {
    pub fn new(threshold: f64, window: usize) -> Self {
        Self {
            history: Vec::new(),
            threshold,
            window,
        }
    }

    pub fn record(&mut self, value: f64) {
        self.history.push(value);
    }

    /// Check if the last `window` values are within threshold.
    pub fn is_converged(&self) -> bool {
        if self.history.len() < self.window {
            return false;
        }
        let recent: Vec<f64> = self.history.iter().rev().take(self.window).copied().collect();
        let min = recent.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = recent.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (max - min).abs() < self.threshold
    }
}

/// Collective decision via majority vote on ternary values.
pub fn collective_decision(votes: &[Trit]) -> Trit {
    let mut counts = [0i32; 3]; // Neg, Zero, Pos
    for &v in votes {
        match v {
            Trit::Neg => counts[0] += 1,
            Trit::Zero => counts[1] += 1,
            Trit::Pos => counts[2] += 1,
        }
    }
    if counts[0] > counts[1] && counts[0] > counts[2] {
        Trit::Neg
    } else if counts[2] > counts[0] && counts[2] > counts[1] {
        Trit::Pos
    } else {
        Trit::Zero
    }
}

/// Weighted collective decision.
pub fn weighted_decision(votes: &[(Trit, f64)]) -> Trit {
    let mut scores = [0.0f64; 3];
    for &(v, w) in votes {
        match v {
            Trit::Neg => scores[0] += w,
            Trit::Zero => scores[1] += w,
            Trit::Pos => scores[2] += w,
        }
    }
    if scores[0] > scores[1] && scores[0] > scores[2] {
        Trit::Neg
    } else if scores[2] > scores[0] && scores[2] > scores[1] {
        Trit::Pos
    } else {
        Trit::Zero
    }
}

/// Multi-round consensus decision (repeated voting until consensus).
pub fn consensus_round(votes: &[Trit], rounds: usize) -> Trit {
    let mut current = votes.to_vec();
    for _ in 0..rounds {
        let decision = collective_decision(&current);
        // All adopt the majority decision
        current = vec![decision; current.len()];
    }
    collective_decision(&current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trit_values() {
        assert_eq!(Trit::Neg.to_i8(), -1);
        assert_eq!(Trit::Zero.to_i8(), 0);
        assert_eq!(Trit::Pos.to_i8(), 1);
    }

    #[test]
    fn test_trit_from_i8() {
        assert_eq!(Trit::from_i8(-1), Some(Trit::Neg));
        assert_eq!(Trit::from_i8(0), Some(Trit::Zero));
        assert_eq!(Trit::from_i8(1), Some(Trit::Pos));
        assert_eq!(Trit::from_i8(5), None);
    }

    #[test]
    fn test_trit_not() {
        assert_eq!(Trit::Neg.ternary_not(), Trit::Pos);
        assert_eq!(Trit::Zero.ternary_not(), Trit::Zero);
        assert_eq!(Trit::Pos.ternary_not(), Trit::Neg);
    }

    #[test]
    fn test_grid_all_positions() {
        let positions = GridPos::all_positions();
        assert_eq!(positions.len(), 9);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = GridPos::new(Trit::Neg, Trit::Neg);
        let b = GridPos::new(Trit::Pos, Trit::Pos);
        assert_eq!(a.manhattan_distance(b), 4);
    }

    #[test]
    fn test_particle_swarm_convergence() {
        let particles = GridPos::all_positions()
            .into_iter()
            .map(|p| Particle::new(p))
            .collect();
        let fitness = |pos: GridPos| -> f64 {
            (pos.x.to_i8() + pos.y.to_i8()) as f64
        };
        let mut swarm = ParticleSwarm::new(particles, fitness);
        let (best, fit) = swarm.run(50);
        assert!(fit >= 1.0); // should find Pos,Pos
        assert_eq!(best, GridPos::new(Trit::Pos, Trit::Pos));
    }

    #[test]
    fn test_particle_swarm_step() {
        let particles = vec![
            Particle::new(GridPos::new(Trit::Neg, Trit::Neg)),
            Particle::new(GridPos::new(Trit::Pos, Trit::Pos)),
        ];
        let fitness = |pos: GridPos| (pos.x.to_i8().abs() + pos.y.to_i8().abs()) as f64;
        let mut swarm = ParticleSwarm::new(particles, fitness);
        swarm.step();
        // Should still have valid positions
        for p in &swarm.particles {
            let x = p.pos.x.to_i8();
            let y = p.pos.y.to_i8();
            assert!(x >= -1 && x <= 1);
            assert!(y >= -1 && y <= 1);
        }
    }

    #[test]
    fn test_pheromone_grid_deposit() {
        let mut grid = PheromoneGrid::new();
        grid.deposit(&[0, 1, 2, 3], Trit::Pos);
        assert_eq!(grid.get_trail(0, 1), Trit::Pos);
        assert_eq!(grid.get_trail(1, 2), Trit::Pos);
        assert_eq!(grid.get_trail(2, 3), Trit::Pos);
        assert_eq!(grid.get_trail(0, 3), Trit::Zero); // not on path
    }

    #[test]
    fn test_pheromone_evaporate() {
        let mut grid = PheromoneGrid::new();
        grid.set_trail(0, 1, Trit::Pos);
        grid.set_trail(2, 3, Trit::Neg);
        grid.evaporate();
        assert_eq!(grid.get_trail(0, 1), Trit::Zero);
        assert_eq!(grid.get_trail(2, 3), Trit::Zero);
    }

    #[test]
    fn test_ant_colony_basic() {
        let dist = vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 0.0],
        ];
        let mut colony = AntColony::new(3, dist);
        let (path, cost) = colony.run(10);
        assert!(!path.is_empty());
        assert!(cost < f64::INFINITY);
    }

    #[test]
    fn test_ant_visit() {
        let mut ant = Ant::new(0, 3);
        assert!(!ant.has_visited(1));
        ant.visit(1, 2.5);
        assert!(ant.has_visited(1));
        assert_eq!(ant.total_cost, 2.5);
    }

    #[test]
    fn test_ant_complete() {
        let mut ant = Ant::new(0, 2);
        assert!(!ant.is_complete());
        ant.visit(1, 1.0);
        assert!(ant.is_complete());
    }

    #[test]
    fn test_alignment_rule_roundtrip() {
        assert_eq!(AlignmentRule::from_trit(Trit::Pos).to_trit(), Trit::Pos);
        assert_eq!(AlignmentRule::from_trit(Trit::Neg).to_trit(), Trit::Neg);
        assert_eq!(AlignmentRule::from_trit(Trit::Zero).to_trit(), Trit::Zero);
    }

    #[test]
    fn test_flock_step() {
        let boids = vec![
            Boid::new(GridPos::new(Trit::Neg, Trit::Neg), AlignmentRule::Align),
            Boid::new(GridPos::new(Trit::Pos, Trit::Pos), AlignmentRule::Align),
            Boid::new(GridPos::new(Trit::Zero, Trit::Zero), AlignmentRule::Align),
        ];
        let mut flock = Flock::new(boids);
        flock.step();
        // All should still be valid positions
        for b in &flock.boids {
            let x = b.pos.x.to_i8();
            assert!(x >= -1 && x <= 1);
        }
    }

    #[test]
    fn test_flock_scatter() {
        let boids = vec![
            Boid::new(GridPos::new(Trit::Zero, Trit::Zero), AlignmentRule::Scatter),
            Boid::new(GridPos::new(Trit::Zero, Trit::Zero), AlignmentRule::Scatter),
        ];
        let mut flock = Flock::new(boids);
        flock.step();
        // Scatter boids should move opposite to majority (which is Zero -> stay)
    }

    #[test]
    fn test_convergence_detector() {
        let mut det = ConvergenceDetector::new(0.01, 3);
        det.record(1.0);
        assert!(!det.is_converged());
        det.record(1.0);
        det.record(1.0);
        assert!(det.is_converged());
    }

    #[test]
    fn test_convergence_detector_not_yet() {
        let mut det = ConvergenceDetector::new(0.01, 3);
        det.record(1.0);
        det.record(2.0);
        det.record(3.0);
        assert!(!det.is_converged());
    }

    #[test]
    fn test_collective_decision_pos() {
        let votes = vec![Trit::Pos, Trit::Pos, Trit::Neg, Trit::Zero];
        assert_eq!(collective_decision(&votes), Trit::Pos);
    }

    #[test]
    fn test_collective_decision_tie() {
        let votes = vec![Trit::Pos, Trit::Neg];
        assert_eq!(collective_decision(&votes), Trit::Zero);
    }

    #[test]
    fn test_weighted_decision() {
        let votes = vec![(Trit::Neg, 0.1), (Trit::Pos, 0.9), (Trit::Zero, 0.5)];
        assert_eq!(weighted_decision(&votes), Trit::Pos);
    }

    #[test]
    fn test_consensus_round() {
        let votes = vec![Trit::Pos, Trit::Pos, Trit::Neg];
        let result = consensus_round(&votes, 3);
        assert_eq!(result, Trit::Pos);
    }

    #[test]
    fn test_swarm_is_converged() {
        let particles = vec![
            Particle::new(GridPos::new(Trit::Zero, Trit::Zero)),
            Particle::new(GridPos::new(Trit::Zero, Trit::Zero)),
        ];
        let fitness = |_| 1.0;
        let swarm = ParticleSwarm::new(particles, fitness);
        assert!(swarm.is_converged());
    }
}
