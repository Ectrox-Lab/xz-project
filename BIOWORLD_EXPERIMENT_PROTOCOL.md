# BIOWORLD_EXPERIMENT_PROTOCOL

## Run
```bash
cd bioworld_mvp
./run_experiment.sh 1 10000 16
```

## Matrix
- A survival (`runs/a_survival`)
- B evolution (`runs/b_evolution`)
- C pressure low/high (`runs/c_pressure_low`, `runs/c_pressure_high`)
- D cooperation (`runs/d_cooperation`)
- E akashic off/on (`runs/e_akashic_off`, `runs/e_akashic_on`)
- F cross-seed robustness is represented by multi-universe batch summary (`runs/*/cross_seed_summary.csv`)

## Required outputs
Top-level: `runs/population.csv`, `runs/cdi.csv`, `runs/mutation.csv`, `runs/boss.csv`, `runs/extinction.csv`, `runs/summary.json`.
Audit: `bioworld_mvp/audit/report.json`.
