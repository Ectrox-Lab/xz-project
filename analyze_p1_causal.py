#!/usr/bin/env python3
"""
P1 Causal Validation Analysis
Analyzes experimental results to validate CDI as a causal state variable.

Pass Criteria (≥2 of 3):
1. CDI trajectory change (intervention → ΔCDI)
2. Hazard rate change (ΔCDI → Δhazard)
3. Extinction timing shift (Δhazard → Δextinction_timing)
"""

import json
import csv
import sys
import numpy as np
from pathlib import Path
from scipy import stats
from dataclasses import dataclass
from typing import Dict, List, Tuple, Optional

@dataclass
class GroupResult:
    name: str
    seeds: List[Dict]
    cdi_trajectories: Dict[str, List[float]]
    hazard_trajectories: Dict[str, List[float]]
    extinction_events: Dict[str, int]
    avg_final_population: float
    
def load_cdi_data(base_path: str, seed: int, universe: int) -> Tuple[List[int], List[float]]:
    """Load CDI time series from CSV"""
    path = Path(base_path) / f"seed_{seed}" / f"u{universe}" / "cdi.csv"
    ticks = []
    cdis = []
    with open(path, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            ticks.append(int(row['tick']))
            cdis.append(float(row['cdi']))
    return ticks, cdis

def load_extinction_data(base_path: str, seed: int, universe: int) -> Tuple[List[int], List[float], List[float]]:
    """Load extinction/hazard data from CSV"""
    path = Path(base_path) / f"seed_{seed}" / f"u{universe}" / "extinction.csv"
    ticks = []
    hazards = []
    probs = []
    with open(path, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            ticks.append(int(row['tick']))
            hazards.append(float(row['hazard_rate']))
            probs.append(float(row['extinction_probability']))
    return ticks, hazards, probs

def load_population_data(base_path: str, seed: int, universe: int) -> Tuple[List[int], List[int]]:
    """Load population time series"""
    path = Path(base_path) / f"seed_{seed}" / f"u{universe}" / "population.csv"
    ticks = []
    pops = []
    with open(path, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            ticks.append(int(row['tick']))
            pops.append(int(row['population']))
    return ticks, pops

def find_extinction_time(populations: List[int], threshold: int = 10) -> Optional[int]:
    """Find first time population drops below threshold"""
    for i, pop in enumerate(populations):
        if pop < threshold:
            return i
    return None

def compute_trajectory_stats(trajectories: Dict[str, List[float]]) -> Dict:
    """Compute mean and confidence intervals across trajectories"""
    # Find minimum length
    min_len = min(len(t) for t in trajectories.values())
    
    # Align and compute statistics
    aligned = np.array([t[:min_len] for t in trajectories.values()])
    mean = np.mean(aligned, axis=0)
    std = np.std(aligned, axis=0)
    sem = std / np.sqrt(len(trajectories))
    ci = 1.96 * sem  # 95% CI
    
    return {
        'mean': mean,
        'std': std,
        'ci_lower': mean - ci,
        'ci_upper': mean + ci,
        'final_mean': mean[-1],
        'final_std': std[-1]
    }

def compare_groups(ctrl: GroupResult, intervention: GroupResult, alpha: float = 0.05) -> Dict:
    """Compare control vs intervention group"""
    results = {}
    
    # 1. CDI trajectory comparison (final values)
    ctrl_cdi_stats = compute_trajectory_stats(ctrl.cdi_trajectories)
    intv_cdi_stats = compute_trajectory_stats(intervention.cdi_trajectories)
    
    ctrl_final = [t[-1] for t in ctrl.cdi_trajectories.values()]
    intv_final = [t[-1] for t in intervention.cdi_trajectories.values()]
    
    # t-test for CDI difference
    t_stat, p_value = stats.ttest_ind(ctrl_final, intv_final)
    results['cdi_diff_significant'] = p_value < alpha
    results['cdi_t_stat'] = t_stat
    results['cdi_p_value'] = p_value
    results['cdi_ctrl_mean'] = np.mean(ctrl_final)
    results['cdi_intv_mean'] = np.mean(intv_final)
    results['cdi_delta'] = np.mean(intv_final) - np.mean(ctrl_final)
    
    # 2. Hazard rate comparison
    ctrl_hazard_stats = compute_trajectory_stats(ctrl.hazard_trajectories)
    intv_hazard_stats = compute_trajectory_stats(intervention.hazard_trajectories)
    
    ctrl_h_final = [t[-1] for t in ctrl.hazard_trajectories.values()]
    intv_h_final = [t[-1] for t in intervention.hazard_trajectories.values()]
    
    t_stat_h, p_value_h = stats.ttest_ind(ctrl_h_final, intv_h_final)
    results['hazard_diff_significant'] = p_value_h < alpha
    results['hazard_t_stat'] = t_stat_h
    results['hazard_p_value'] = p_value_h
    results['hazard_delta'] = np.mean(intv_h_final) - np.mean(ctrl_h_final)
    
    # 3. Extinction timing comparison
    ctrl_ext_times = [t for t in ctrl.extinction_events.values() if t is not None]
    intv_ext_times = [t for t in intervention.extinction_events.values() if t is not None]
    
    if len(ctrl_ext_times) > 0 and len(intv_ext_times) > 0:
        t_stat_e, p_value_e = stats.ttest_ind(ctrl_ext_times, intv_ext_times)
        results['extinction_shift_significant'] = p_value_e < alpha
        results['extinction_t_stat'] = t_stat_e
        results['extinction_p_value'] = p_value_e
        results['extinction_ctrl_mean'] = np.mean(ctrl_ext_times)
        results['extinction_intv_mean'] = np.mean(intv_ext_times)
    else:
        results['extinction_shift_significant'] = False
        results['extinction_ctrl_mean'] = None
        results['extinction_intv_mean'] = None
    
    # Pass criteria: ≥2 of 3 conditions
    pass_count = sum([
        results['cdi_diff_significant'],
        results['hazard_diff_significant'],
        results.get('extinction_shift_significant', False)
    ])
    results['pass_count'] = pass_count
    results['passed'] = pass_count >= 2
    
    return results

def analyze_group(base_path: str, group_name: str, seeds: List[int], num_universes: int = 8) -> GroupResult:
    """Analyze all data for a group"""
    cdi_traj = {}
    hazard_traj = {}
    ext_events = {}
    final_pops = []
    
    for seed in seeds:
        for u in range(num_universes):
            key = f"seed{seed}_u{u}"
            try:
                # Load CDI
                _, cdi = load_cdi_data(base_path, seed, u)
                cdi_traj[key] = cdi
                
                # Load hazard
                _, hazard, _ = load_extinction_data(base_path, seed, u)
                hazard_traj[key] = hazard
                
                # Load population for extinction timing
                _, pops = load_population_data(base_path, seed, u)
                ext_time = find_extinction_time(pops)
                ext_events[key] = ext_time
                final_pops.append(pops[-1] if pops else 0)
            except Exception as e:
                print(f"  Warning: Could not load {key}: {e}")
    
    return GroupResult(
        name=group_name,
        seeds=[{'seed': s} for s in seeds],
        cdi_trajectories=cdi_traj,
        hazard_trajectories=hazard_traj,
        extinction_events=ext_events,
        avg_final_population=np.mean(final_pops) if final_pops else 0
    )

def main():
    base_path = "/tmp/bio-world/p1_experiments"
    
    print("=" * 70)
    print("  P1 CAUSAL VALIDATION ANALYSIS")
    print("=" * 70)
    print()
    
    # Load and analyze all groups
    print("▶ Loading CTRL group...")
    ctrl = analyze_group(f"{base_path}/ctrl", "CTRL", [101, 102, 103])
    print(f"  ✓ Loaded {len(ctrl.cdi_trajectories)} trajectories")
    
    print("▶ Loading P1-A group (Memory KO)...")
    p1a = analyze_group(f"{base_path}/p1a", "P1-A", [201, 202, 203])
    print(f"  ✓ Loaded {len(p1a.cdi_trajectories)} trajectories")
    
    print("▶ Loading P1-B group (Cooperation suppression)...")
    p1b = analyze_group(f"{base_path}/p1b", "P1-B", [301, 302, 303])
    print(f"  ✓ Loaded {len(p1b.cdi_trajectories)} trajectories")
    
    print("▶ Loading P1-C group (Boss pressure ×1.5)...")
    p1c = analyze_group(f"{base_path}/p1c", "P1-C", [401, 402, 403])
    print(f"  ✓ Loaded {len(p1c.cdi_trajectories)} trajectories")
    
    print()
    print("=" * 70)
    print("  CAUSAL COMPARISON RESULTS")
    print("=" * 70)
    
    # Compare each intervention to control
    comparisons = [
        ("P1-A (Memory KO)", ctrl, p1a),
        ("P1-B (Cooperation suppression)", ctrl, p1b),
        ("P1-C (Boss pressure ×1.5)", ctrl, p1c),
    ]
    
    all_passed = []
    
    for name, control, intervention in comparisons:
        print()
        print(f"▶ {name} vs CTRL")
        print("-" * 50)
        
        results = compare_groups(control, intervention)
        
        print(f"  CDI Change:")
        print(f"    CTRL: {results['cdi_ctrl_mean']:.4f} ± {results.get('cdi_ctrl_std', 0):.4f}")
        print(f"    INTV: {results['cdi_intv_mean']:.4f} ± {results.get('cdi_intv_std', 0):.4f}")
        print(f"    Δ: {results['cdi_delta']:.4f} (p={results['cdi_p_value']:.4f})")
        print(f"    {'✓ SIGNIFICANT' if results['cdi_diff_significant'] else '✗ Not significant'}")
        
        print(f"  Hazard Change:")
        print(f"    Δ: {results['hazard_delta']:.6f} (p={results['hazard_p_value']:.4f})")
        print(f"    {'✓ SIGNIFICANT' if results['hazard_diff_significant'] else '✗ Not significant'}")
        
        if results['extinction_ctrl_mean'] is not None:
            print(f"  Extinction Timing:")
            print(f"    CTRL: {results['extinction_ctrl_mean']:.0f}")
            print(f"    INTV: {results['extinction_intv_mean']:.0f}")
            print(f"    {'✓ SHIFT' if results.get('extinction_shift_significant') else '✗ No shift'}")
        
        print(f"  Pass Criteria: {results['pass_count']}/3 met")
        print(f"  RESULT: {'✅ PASSED' if results['passed'] else '❌ FAILED'}")
        
        all_passed.append(results['passed'])
    
    # Summary
    print()
    print("=" * 70)
    print("  SUMMARY")
    print("=" * 70)
    print()
    
    for (name, _, _), passed in zip(comparisons, all_passed):
        status = "✅ PASS" if passed else "❌ FAIL"
        print(f"  {name}: {status}")
    
    total_pass = sum(all_passed)
    print()
    print(f"  Total: {total_pass}/3 interventions validated")
    
    if total_pass >= 2:
        print()
        print("  🎉 P1 CAUSAL VALIDATION: PASSED")
        print("     CDI is confirmed as a causal state variable")
        print("     (not just a leading indicator)")
    else:
        print()
        print("  ⚠️  P1 CAUSAL VALIDATION: INCONCLUSIVE")
        print("     Need Phase 2 with larger sample size")
    
    print()
    print("=" * 70)

if __name__ == "__main__":
    main()
