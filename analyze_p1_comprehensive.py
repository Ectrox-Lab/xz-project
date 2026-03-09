#!/usr/bin/env python3
"""
P1 Causal Validation - Comprehensive Analysis
Uses multiple metrics to validate causal relationships:
- Signal diversity (complexity proxy)
- Memory usage (L1 cell memory utilization)
- Death rate (extinction risk)
- Population trajectory
- Multi-boss success rate (cooperation proxy)
"""

import json
import csv
import sys
import numpy as np
from pathlib import Path
from scipy import stats
from dataclasses import dataclass
from typing import Dict, List, Tuple, Optional
import warnings
warnings.filterwarnings('ignore')

@dataclass
class UniverseData:
    name: str
    ticks: List[int]
    signal_diversity: List[float]
    cooperation_density: List[float]
    memory_usage: List[float]
    exploration_rate: List[float]
    cdi: List[float]
    death_rate: List[float]
    hazard_rate: List[float]
    population: List[int]
    
    @property
    def final_population(self) -> int:
        return self.population[-1] if self.population else 0
    
    @property
    def mean_signal_diversity(self) -> float:
        return np.mean(self.signal_diversity) if self.signal_diversity else 0
    
    @property
    def final_memory_usage(self) -> float:
        return self.memory_usage[-1] if self.memory_usage else 0
    
    @property
    def extinction_time(self) -> Optional[int]:
        """Find when population drops below threshold"""
        threshold = 50
        for i, pop in enumerate(self.population):
            if pop < threshold:
                return self.ticks[i] if i < len(self.ticks) else i
        return None
    
    @property
    def avg_death_rate(self) -> float:
        return np.mean(self.death_rate) if self.death_rate else 0

@dataclass
class GroupData:
    name: str
    universes: Dict[str, UniverseData]
    
    def get_metric(self, metric_name: str) -> List[float]:
        """Extract metric across all universes"""
        values = []
        for u in self.universes.values():
            if metric_name == "final_population":
                values.append(u.final_population)
            elif metric_name == "mean_signal_diversity":
                values.append(u.mean_signal_diversity)
            elif metric_name == "final_memory_usage":
                values.append(u.final_memory_usage)
            elif metric_name == "avg_death_rate":
                values.append(u.avg_death_rate)
            elif metric_name == "extinction_time":
                if u.extinction_time is not None:
                    values.append(u.extinction_time)
        return values

def load_universe_data(base_path: str, seed: int, universe: int) -> Optional[UniverseData]:
    """Load all data for a single universe"""
    name = f"seed{seed}_u{universe}"
    
    try:
        # Load CDI data
        cdi_path = Path(base_path) / f"seed_{seed}" / f"u{universe}" / "cdi.csv"
        ticks, signal_div, coop_dens, mem_use, explore, cdi = [], [], [], [], [], []
        with open(cdi_path, 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                ticks.append(int(row['tick']))
                signal_div.append(float(row['signal_diversity']))
                coop_dens.append(float(row['cooperation_density']))
                mem_use.append(float(row['memory_usage']))
                explore.append(float(row['exploration_rate']))
                cdi.append(float(row['cdi']))
        
        # Load extinction data
        ext_path = Path(base_path) / f"seed_{seed}" / f"u{universe}" / "extinction.csv"
        death_rate, hazard_rate = [], []
        with open(ext_path, 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                death_rate.append(float(row['death_rate']))
                hazard_rate.append(float(row['hazard_rate']))
        
        # Load population data
        pop_path = Path(base_path) / f"seed_{seed}" / f"u{universe}" / "population.csv"
        population = []
        with open(pop_path, 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                population.append(int(row['population']))
        
        return UniverseData(
            name=name,
            ticks=ticks,
            signal_diversity=signal_div,
            cooperation_density=coop_dens,
            memory_usage=mem_use,
            exploration_rate=explore,
            cdi=cdi,
            death_rate=death_rate,
            hazard_rate=hazard_rate,
            population=population
        )
    except Exception as e:
        print(f"  Warning: Could not load {name}: {e}")
        return None

def load_group(base_path: str, seeds: List[int], num_universes: int = 8) -> GroupData:
    """Load all data for a group"""
    universes = {}
    for seed in seeds:
        for u in range(num_universes):
            data = load_universe_data(base_path, seed, u)
            if data:
                universes[data.name] = data
    return GroupData(name=Path(base_path).name, universes=universes)

def compare_metric(ctrl_values: List[float], intv_values: List[float], metric_name: str) -> Dict:
    """Compare a metric between control and intervention"""
    if not ctrl_values or not intv_values:
        return {'significant': False, 'delta': 0, 'p_value': 1.0, 'effect_size': 0}
    
    # t-test
    t_stat, p_value = stats.ttest_ind(ctrl_values, intv_values)
    
    # Cohen's d (effect size)
    pooled_std = np.sqrt((np.std(ctrl_values)**2 + np.std(intv_values)**2) / 2)
    cohens_d = (np.mean(intv_values) - np.mean(ctrl_values)) / pooled_std if pooled_std > 0 else 0
    
    return {
        'ctrl_mean': np.mean(ctrl_values),
        'ctrl_std': np.std(ctrl_values),
        'intv_mean': np.mean(intv_values),
        'intv_std': np.std(intv_values),
        'delta': np.mean(intv_values) - np.mean(ctrl_values),
        'delta_pct': ((np.mean(intv_values) - np.mean(ctrl_values)) / abs(np.mean(ctrl_values)) * 100) if np.mean(ctrl_values) != 0 else 0,
        't_stat': t_stat,
        'p_value': p_value,
        'significant': p_value < 0.05,
        'effect_size': cohens_d,
        'effect_magnitude': 'large' if abs(cohens_d) > 0.8 else ('medium' if abs(cohens_d) > 0.5 else 'small')
    }

def print_comparison(name: str, result: Dict):
    """Print comparison results"""
    sig_marker = "✓" if result['significant'] else "✗"
    print(f"  {name}:")
    print(f"    CTRL: {result['ctrl_mean']:.4f} ± {result['ctrl_std']:.4f}")
    print(f"    INTV: {result['intv_mean']:.4f} ± {result['intv_std']:.4f}")
    print(f"    Δ: {result['delta']:.4f} ({result['delta_pct']:+.1f}%)")
    print(f"    p-value: {result['p_value']:.4f} {sig_marker}")
    print(f"    Effect size (Cohen's d): {result['effect_size']:.3f} ({result['effect_magnitude']})")

def analyze_intervention(ctrl: GroupData, intervention: GroupData, intervention_name: str) -> Dict:
    """Full analysis of an intervention vs control"""
    print()
    print(f"▶ {intervention_name} vs CTRL")
    print("-" * 60)
    
    results = {}
    
    # Metric 1: Signal diversity (complexity proxy)
    results['signal_diversity'] = compare_metric(
        ctrl.get_metric('mean_signal_diversity'),
        intervention.get_metric('mean_signal_diversity'),
        'Signal Diversity'
    )
    print_comparison("Signal Diversity (Complexity Proxy)", results['signal_diversity'])
    
    # Metric 2: Memory usage
    results['memory_usage'] = compare_metric(
        ctrl.get_metric('final_memory_usage'),
        intervention.get_metric('final_memory_usage'),
        'Memory Usage'
    )
    print_comparison("Final Memory Usage", results['memory_usage'])
    
    # Metric 3: Death rate
    results['death_rate'] = compare_metric(
        ctrl.get_metric('avg_death_rate'),
        intervention.get_metric('avg_death_rate'),
        'Death Rate'
    )
    print_comparison("Average Death Rate", results['death_rate'])
    
    # Metric 4: Final population
    results['population'] = compare_metric(
        ctrl.get_metric('final_population'),
        intervention.get_metric('final_population'),
        'Final Population'
    )
    print_comparison("Final Population", results['population'])
    
    # Metric 5: Extinction timing (if applicable)
    ctrl_ext = ctrl.get_metric('extinction_time')
    intv_ext = intervention.get_metric('extinction_time')
    if ctrl_ext or intv_ext:
        results['extinction'] = compare_metric(ctrl_ext, intv_ext, 'Extinction Time')
        print_comparison("Extinction Time", results['extinction'])
    
    # Pass criteria for P1
    # At least 2 of: signal diversity change, death rate change, population change
    pass_count = sum([
        results['signal_diversity']['significant'] and abs(results['signal_diversity']['effect_size']) > 0.3,
        results['death_rate']['significant'] and abs(results['death_rate']['effect_size']) > 0.3,
        results['population']['significant'] and abs(results['population']['effect_size']) > 0.3
    ])
    
    results['pass_count'] = pass_count
    results['passed'] = pass_count >= 2
    
    print()
    print(f"  Pass Criteria: {pass_count}/3 significant effects with |d|>0.3")
    print(f"  RESULT: {'✅ PASSED' if results['passed'] else '❌ FAILED'}")
    
    return results

def plot_trajectories(ctrl: GroupData, interventions: List[Tuple[str, GroupData]], output_path: str = "/tmp/bio-world/p1_trajectories.png"):
    """Generate trajectory plots"""
    try:
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
        
        fig, axes = plt.subplots(2, 2, figsize=(14, 10))
        
        metrics = [
            ('signal_diversity', 'Signal Diversity', axes[0, 0]),
            ('memory_usage', 'Memory Usage', axes[0, 1]),
            ('death_rate', 'Death Rate', axes[1, 0]),
            ('population', 'Population', axes[1, 1]),
        ]
        
        colors = {'CTRL': 'blue', 'P1-A': 'red', 'P1-B': 'green', 'P1-C': 'orange'}
        
        for metric_key, metric_label, ax in metrics:
            # Plot control
            for u in list(ctrl.universes.values())[:3]:  # Sample 3 universes
                if metric_key == 'signal_diversity':
                    values = u.signal_diversity
                elif metric_key == 'memory_usage':
                    values = u.memory_usage
                elif metric_key == 'death_rate':
                    values = u.death_rate
                else:
                    values = u.population
                ax.plot(u.ticks[:len(values)], values, color=colors['CTRL'], alpha=0.3, linewidth=0.5)
            
            # Plot interventions
            for name, group in interventions:
                for u in list(group.universes.values())[:3]:
                    if metric_key == 'signal_diversity':
                        values = u.signal_diversity
                    elif metric_key == 'memory_usage':
                        values = u.memory_usage
                    elif metric_key == 'death_rate':
                        values = u.death_rate
                    else:
                        values = u.population
                    ax.plot(u.ticks[:len(values)], values, color=colors.get(name, 'gray'), alpha=0.3, linewidth=0.5)
            
            ax.set_xlabel('Generation')
            ax.set_ylabel(metric_label)
            ax.set_title(f'{metric_label} Trajectories')
            ax.legend(['CTRL', 'P1-A', 'P1-B', 'P1-C'], loc='best')
        
        plt.tight_layout()
        plt.savefig(output_path, dpi=150)
        print(f"  Plot saved to: {output_path}")
    except Exception as e:
        print(f"  Could not generate plot: {e}")

def main():
    base_path = "/tmp/bio-world/p1_experiments"
    
    print("=" * 70)
    print("  P1 CAUSAL VALIDATION - COMPREHENSIVE ANALYSIS")
    print("=" * 70)
    print()
    
    # Load all groups
    print("▶ Loading data...")
    ctrl = load_group(f"{base_path}/ctrl", [101, 102, 103])
    print(f"  CTRL: {len(ctrl.universes)} universes loaded")
    
    p1a = load_group(f"{base_path}/p1a", [201, 202, 203])
    print(f"  P1-A: {len(p1a.universes)} universes loaded")
    
    p1b = load_group(f"{base_path}/p1b", [301, 302, 303])
    print(f"  P1-B: {len(p1b.universes)} universes loaded")
    
    p1c = load_group(f"{base_path}/p1c", [401, 402, 403])
    print(f"  P1-C: {len(p1c.universes)} universes loaded")
    
    print()
    print("=" * 70)
    print("  INTERVENTION ANALYSIS")
    print("=" * 70)
    
    # Analyze each intervention
    results = {}
    results['P1-A'] = analyze_intervention(ctrl, p1a, "P1-A (Memory KO)")
    results['P1-B'] = analyze_intervention(ctrl, p1b, "P1-B (Cooperation suppression)")
    results['P1-C'] = analyze_intervention(ctrl, p1c, "P1-C (Boss pressure ×1.5)")
    
    # Generate plots
    print()
    print("▶ Generating trajectory plots...")
    plot_trajectories(ctrl, [('P1-A', p1a), ('P1-B', p1b), ('P1-C', p1c)])
    
    # Summary
    print()
    print("=" * 70)
    print("  SUMMARY")
    print("=" * 70)
    print()
    
    for name, result in results.items():
        status = "✅ PASS" if result['passed'] else "❌ FAIL"
        print(f"  {name}: {status} ({result['pass_count']}/3 criteria)")
    
    total_pass = sum(1 for r in results.values() if r['passed'])
    print()
    print(f"  Total: {total_pass}/3 interventions validated")
    
    if total_pass >= 2:
        print()
        print("  🎉 P1 CAUSAL VALIDATION: PASSED")
        print("     Multiple causal pathways confirmed:")
        if results['P1-A']['passed']:
            print("     • Memory → Complexity (structural causality)")
        if results['P1-B']['passed']:
            print("     • Cooperation → Population dynamics")
        if results['P1-C']['passed']:
            print("     • Environmental stress → System stability")
    else:
        print()
        print("  ⚠️  P1 CAUSAL VALIDATION: INCONCLUSIVE")
        print("     Need Phase 2 with larger sample size")
    
    print()
    print("=" * 70)

if __name__ == "__main__":
    main()
