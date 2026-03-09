use crate::bio_world::engine::dna::{Dna, MutationEvent};
use crate::bio_world::engine::world::Rng;

pub fn mutate_dna(dna: &Dna, rng: &mut Rng, tick: u32, lineage_id: u64) -> (Dna, Vec<MutationEvent>) {
    let mut out = dna.clone();
    let mut ev = Vec::new();

    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.08); out.move_speed = (out.move_speed + d).clamp(0.05, 1.0); ev.push(MutationEvent{tick,lineage_id,parameter:"move_speed",delta:d}); }
    if rng.bool(dna.mutation_rate) { out.sensing_radius = (out.sensing_radius + rng.range_i32(-1, 2)).clamp(1, 8); ev.push(MutationEvent{tick,lineage_id,parameter:"sensing_radius",delta:0.0}); }
    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.08); out.attack_power = (out.attack_power + d).clamp(0.05, 1.0); ev.push(MutationEvent{tick,lineage_id,parameter:"attack_power",delta:d}); }
    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.08); out.defense = (out.defense + d).clamp(0.05, 1.0); ev.push(MutationEvent{tick,lineage_id,parameter:"defense",delta:d}); }
    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.08); out.cooperation_willingness = (out.cooperation_willingness + d).clamp(0.0, 1.0); ev.push(MutationEvent{tick,lineage_id,parameter:"cooperation_willingness",delta:d}); }
    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.08); out.signal_strength = (out.signal_strength + d).clamp(0.0, 1.0); ev.push(MutationEvent{tick,lineage_id,parameter:"signal_strength",delta:d}); }
    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.08); out.signal_frequency = (out.signal_frequency + d).clamp(0.0, 1.0); ev.push(MutationEvent{tick,lineage_id,parameter:"signal_frequency",delta:d}); }
    if rng.bool(dna.mutation_rate) { out.memory_capacity = (out.memory_capacity as i32 + rng.range_i32(-2, 3)).clamp(2, 16) as usize; ev.push(MutationEvent{tick,lineage_id,parameter:"memory_capacity",delta:0.0}); }
    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.08); out.learning_rate = (out.learning_rate + d).clamp(0.0, 1.0); ev.push(MutationEvent{tick,lineage_id,parameter:"learning_rate",delta:d}); }
    if rng.bool(dna.mutation_rate) { let d = rng.normal(0.04); out.mutation_rate = (out.mutation_rate + d).clamp(0.001, 0.25); ev.push(MutationEvent{tick,lineage_id,parameter:"mutation_rate",delta:d}); }

    (out, ev)
}
