# Phase 4 Command Replay

## Build
```bash
cd /workspace/bio-world/bioworld_mvp
cargo build --release
```

## Core runs
```bash
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode baseline_full
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode no_L2
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode L3_off
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode L3_real_p001
./target/release/bioworld_mvp --ticks 2000 --universes 4 --sentinel-mode L3_shuffled_p001
```

## Aggregate compare (real vs shuffled)
```bash
python - <<'PY'
import csv, statistics, pathlib
base=pathlib.Path('/workspace/bio-world/bioworld_mvp/runs/sentinel')
metrics=['lineage_diversity','top1_lineage_share','strategy_entropy','collapse_event_count']

def means(cond):
    vals={m:[] for m in metrics}
    with open(base/cond/'per_run.csv') as f:
        r=csv.DictReader(f)
        for row in r:
            for m in metrics:
                vals[m].append(float(row[m]))
    return {m:statistics.fmean(vals[m]) for m in metrics}

real=means('L3_real_p001')
shuf=means('L3_shuffled_p001')
for m in metrics:
    print(f"{m}: real={real[m]:.6f} shuffled={shuf[m]:.6f} delta={real[m]-shuf[m]:.6f}")
PY
```
