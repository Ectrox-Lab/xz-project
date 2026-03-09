# Phase 3 Reproducible Commands

## Environment
```bash
cd /workspace/bio-world/bioworld_mvp
cargo build --release
```

## Core sentinel runs (executed)
```bash
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode baseline_full
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode no_L2
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode L3_off
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode L3_real_p001
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode L3_shuffled_p001
```

## Optional full suite
```bash
./target/release/bioworld_mvp --ticks 2000 --universes 4 --run-sentinel-suite
```

## Compare L3_real_p001 vs L3_shuffled_p001 (mean deltas)
```bash
python - <<'PY'
import csv, statistics, pathlib
base=pathlib.Path('/workspace/bio-world/bioworld_mvp/runs/sentinel')
metrics=['lineage_diversity','top1_lineage_share','strategy_entropy','collapse_event_count']

def means(path):
    vals={m:[] for m in metrics}
    with open(path) as f:
        r=csv.DictReader(f)
        for row in r:
            for m in metrics:
                vals[m].append(float(row[m]))
    return {m: (statistics.fmean(vals[m]) if vals[m] else 0.0) for m in metrics}

real=means(base/'L3_real_p001'/'per_run.csv')
shuf=means(base/'L3_shuffled_p001'/'per_run.csv')
for m in metrics:
    print(f"{m}: real={real[m]:.6f} shuffled={shuf[m]:.6f} delta={real[m]-shuf[m]:.6f}")
PY
```

## Output locations
- Per-condition run summary: `bioworld_mvp/runs/sentinel/<condition>/per_run.csv`
- Per-seed tick telemetry: `bioworld_mvp/runs/sentinel/<condition>/u*/memory.csv`
