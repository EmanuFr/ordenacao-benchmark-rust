use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::time::{Duration, Instant};

/// Semente fixa: qualquer pessoa que rode o código gera exatamente os mesmos vetores.
/// (Os tempos variam de máquina para máquina, mas as entradas são sempre as mesmas.)
const SEMENTE: u64 = 42;

pub fn novo_rng() -> StdRng {
    StdRng::seed_from_u64(SEMENTE)
}

pub fn gerar_vetor(rng: &mut StdRng, tamanho: usize, max: i32) -> Vec<i32> {
    (0..tamanho).map(|_| rng.random_range(0..max)).collect()
}

pub fn medir(original: &[i32], algoritmo: fn(&mut [i32]) -> &mut [i32]) -> Duration {
    let mut copia = original.to_vec(); // a cópia não entra no tempo

    let inicio = Instant::now();
    algoritmo(&mut copia);
    let tempo = inicio.elapsed();

    // Fora do cronômetro: garante que o algoritmo realmente ordenou o vetor.
    assert!(copia.is_sorted(), "o algoritmo não ordenou o vetor");
    tempo
}
