mod metodos_de_ordenacao;
mod utils;

use crate::metodos_de_ordenacao::{bubble, insertion, merge, selection};
use crate::utils::{gerar_vetor, medir, novo_rng};
use rand::rngs::StdRng;
use std::time::Duration;

/// Roda uma bateria: r vetores aleatórios de tamanho n, com valores em 0..max.
/// Cada vetor é ordenado pelos 4 algoritmos. Retorna a média de cada um,
/// na ordem [Selection, Insertion, Bubble, Merge].
fn bateria(rng: &mut StdRng, n: usize, r: u32, max: i32) -> [Duration; 4] {
    let mut totais = [Duration::ZERO; 4];

    for _ in 0..r {
        let original = gerar_vetor(rng, n, max);

        totais[0] += medir(&original, selection);
        totais[1] += medir(&original, insertion);
        totais[2] += medir(&original, bubble);
        totais[3] += medir(&original, merge);
    }

    totais.map(|t| t / r)
}

fn main() {
    let mut rng = novo_rng();

    // Aquecimento (resultado descartado): deixa CPU e caches em regime
    // antes da primeira medição. Não aparece em nenhuma tabela.
    bateria(&mut rng, 2_000, 200, 100_000);

    // ---------- PARTE 1: bateria obrigatória (n, r) ----------
    let cenarios: [(usize, u32); 3] = [(1_000, 10_000), (2_000, 2_500), (4_000, 625)];

    println!("=== PARTE 1: Tempo médio por algoritmo (valores em 0..100000) ===");
    println!(
        "{:>6} {:>8} | {:>12} {:>12} {:>12} {:>12}",
        "n", "r", "Selection", "Insertion", "Bubble", "Merge"
    );
    println!("{}", "-".repeat(72));

    for (n, r) in cenarios {
        let m = bateria(&mut rng, n, r, 100_000);
        println!(
            "{:>6} {:>8} | {:>12} {:>12} {:>12} {:>12}",
            n,
            r,
            format!("{:?}", m[0]),
            format!("{:?}", m[1]),
            format!("{:?}", m[2]),
            format!("{:?}", m[3])
        );
    }

    // ---------- PARTE 2 (PLUS): reduzindo o intervalo dos valores ----------
    let n = 4_000;
    let r = 200; // menos realizações para o plus não demorar demais
    let intervalos: [i32; 5] = [100_000, 1_000, 100, 10, 2]; // o primeiro é a referência

    println!();
    println!("=== PARTE 2 (PLUS): n = {}, r = {} ===", n, r);
    println!(
        "{:>9} | {:>12} {:>12} {:>12} {:>12}",
        "valores", "Selection", "Insertion", "Bubble", "Merge"
    );
    println!("{}", "-".repeat(67));

    let mut referencia: Option<[Duration; 4]> = None;

    for max in intervalos {
        let m = bateria(&mut rng, n, r, max);
        let base = *referencia.get_or_insert(m);

        println!(
            "{:>9} | {:>12} {:>12} {:>12} {:>12}",
            format!("0..{}", max),
            format!("{:?}", m[0]),
            format!("{:?}", m[1]),
            format!("{:?}", m[2]),
            format!("{:?}", m[3])
        );

        // Speedup = tempo da referência / tempo atual
        let s = |i: usize| format!("{:.2}x", base[i].as_secs_f64() / m[i].as_secs_f64());
        println!(
            "{:>9} | {:>12} {:>12} {:>12} {:>12}",
            "speedup",
            s(0),
            s(1),
            s(2),
            s(3)
        );
    }
}