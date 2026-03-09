use crate::bio_world::akashic::akashic_archive::AkashicArchive;
use crate::bio_world::engine::dna::Dna;
use crate::bio_world::engine::world::Rng;

pub fn sample_akashic_dna(archive: &AkashicArchive, rng: &mut Rng) -> Option<Dna> {
    if archive.elite_dna.is_empty() || !rng.bool(0.08) {
        return None;
    }
    let idx = (rng.next_u64() as usize) % archive.elite_dna.len();
    Some(archive.elite_dna[idx].dna.clone())
}
