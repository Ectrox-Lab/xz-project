#!/usr/bin/env python3
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RUNS = ROOT / "runs"
SRC = ROOT / "bioworld_mvp" / "src" / "main.rs"


def load_csv(name):
    with (RUNS / f"{name}.csv").open() as f:
        return list(csv.DictReader(f))


def totals(rows, key):
    return sum(int(r[key]) for r in rows)


def first_last(rows, key):
    return float(rows[0][key]), float(rows[-1][key])


def check_no_autowin_shortcut():
    src = SRC.read_text(encoding="utf-8")
    has_success_formula = "attackers >= boss.min_attackers && synchrony > 0.55 && invested > 0.25" in src
    has_autowin_pattern = "if neighbors" in src or "auto win" in src
    return {
        "has_success_formula": has_success_formula,
        "has_autowin_pattern": has_autowin_pattern,
        "pass": has_success_formula and not has_autowin_pattern,
    }


def verify():
    a = load_csv("experiment_a_survival")
    b = load_csv("experiment_b_evolution")
    c_low = load_csv("experiment_c_pressure_low")
    c_high = load_csv("experiment_c_pressure_high")
    d = load_csv("experiment_d_cooperation")
    e_off = load_csv("experiment_e_akashic_off")
    e_on = load_csv("experiment_e_akashic_on")

    a_births, a_deaths = totals(a, "births"), totals(a, "deaths")
    a_pop_var = len({int(r["population"]) for r in a}) > 1

    b_mut = totals(b, "mutation_count")
    b_d0, b_d1 = first_last(b, "dna_variance")
    b_lineage = int(b[-1]["lineage_count"])

    c_deaths_low = totals(c_low, "deaths")
    c_deaths_high = totals(c_high, "deaths")
    with (RUNS / "summary.json").open() as f:
        summary = {x["name"]: x for x in json.load(f)}
    c_shift_low = summary["experiment_c_pressure_low"]["directional_shift_move_taxis"]
    c_shift_high = summary["experiment_c_pressure_high"]["directional_shift_move_taxis"]

    d_single = summary["experiment_d_cooperation"]["boss3_single_success"]
    d_multi = summary["experiment_d_cooperation"]["boss3_multi_success"]

    e_adapt_off = summary["experiment_e_akashic_off"]["final_metrics"]["adaptation_gain"]
    e_adapt_on = summary["experiment_e_akashic_on"]["final_metrics"]["adaptation_gain"]
    e_deaths_off = totals(e_off, "deaths")
    e_deaths_on = totals(e_on, "deaths")
    e_boss_off = summary["experiment_e_akashic_off"]["boss3_multi_success"]
    e_boss_on = summary["experiment_e_akashic_on"]["boss3_multi_success"]

    results = {
        "anti_cheat_code_audit": check_no_autowin_shortcut(),
        "experiment_A_survival": {
            "ticks": len(a),
            "births_total": a_births,
            "deaths_total": a_deaths,
            "population_non_constant": a_pop_var,
            "pass": len(a) >= 10_000 and a_births > 0 and a_deaths > 0 and a_pop_var,
        },
        "experiment_B_evolution": {
            "mutation_count_total": b_mut,
            "dna_variance_start": b_d0,
            "dna_variance_end": b_d1,
            "lineage_count_final": b_lineage,
            "pass": b_mut > 0 and b_d1 > b_d0 and b_lineage > 2,
        },
        "experiment_C_pressure": {
            "deaths_low": c_deaths_low,
            "deaths_high": c_deaths_high,
            "shift_move_taxis_low": c_shift_low,
            "shift_move_taxis_high": c_shift_high,
            "pass": c_deaths_high > c_deaths_low and c_shift_low * c_shift_high < 0,
        },
        "experiment_D_cooperation": {
            "boss3_single_success": d_single,
            "boss3_multi_success": d_multi,
            "pass": d_single <= 0.05 and d_multi > d_single + 0.3,
        },
        "experiment_E_akashic": {
            "adaptation_gain_off": e_adapt_off,
            "adaptation_gain_on": e_adapt_on,
            "deaths_off_total": e_deaths_off,
            "deaths_on_total": e_deaths_on,
            "boss3_multi_success_off": e_boss_off,
            "boss3_multi_success_on": e_boss_on,
            "pass": e_adapt_on > e_adapt_off,
            "note": "Akashic 目标是经验加速，不保证死亡率单调下降。",
        },
    }

    out = ROOT / "runs" / "verification.json"
    out.write_text(json.dumps(results, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps(results, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    verify()
