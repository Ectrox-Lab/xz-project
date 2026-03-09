#!/usr/bin/env python3
from pathlib import Path
src = Path(__file__).resolve().parents[1] / 'src'
text = (src / 'bio_world' / 'engine' / 'world.rs').read_text(encoding='utf-8')
checks = {
    'no_auto_win_neighbors': 'if neighbors >' not in text,
    'no_global_population_read': 'global population' not in text,
    'cooperation_formula_present': 'attackers >= threshold && synchrony > x && signal_investment > y' in (src / 'bio_world' / 'evolution' / 'cooperation.rs').read_text(encoding='utf-8')
}
ok = all(checks.values())
print({'checks': checks, 'pass': ok})
Path('audit').mkdir(exist_ok=True)
Path('audit/report.json').write_text(str({'checks': checks, 'pass': ok}))
if not ok:
    raise SystemExit(1)
