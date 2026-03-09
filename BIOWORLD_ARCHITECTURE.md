# BIOWORLD_ARCHITECTURE

Research-grade modular architecture implemented in `bioworld_mvp/src/bio_world`:
- engine: world/cell/dna/energy/physics
- evolution: mutation/reproduction/selection/cooperation
- boss: boss_system/difficulty_curve (10 levels)
- akashic: archive + cross universe transfer
- metrics: cdi/stability/extinction
- experiments: runner + cross_seed
- output: csv/json

World size: `50x50x16`.

Cooperation success (non-hardcoded):
`attackers >= threshold && synchrony > X && signal_investment > Y`.

CDI:
`signal_diversity * cooperation_density * memory_usage * exploration_rate`.

Hazard:
`P(extinction)=1-exp(-h*dt)` and `h` increases when CDI < I_crit.
