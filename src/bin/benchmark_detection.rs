use octet_econome::{encode_ascii_compact, injecter_attaque, verificateur_noeud_embarque};
use std::time::Instant;

fn run_case(label: &str, flux_size: usize, attack_size: usize, iterations: usize) {
    let start = Instant::now();
    let mut detected = 0usize;
    let mut total_duration = std::time::Duration::new(0, 0);

    for _ in 0..iterations {
        let message = format!("{} {} ", label, "x".repeat(flux_size / 8));
        let mut flux = encode_ascii_compact(&message);
        let reference = flux.clone();

        let attack_start = std::cmp::min(flux.len().saturating_sub(1), flux_size / 2);
        injecter_attaque(&mut flux, attack_start, attack_size.min(flux.len()));

        let t0 = Instant::now();
        let verdict = verificateur_noeud_embarque(&flux, &reference, 64);
        total_duration += t0.elapsed();

        if matches!(verdict.statut, octet_econome::StatutSysteme::CritiqueAttaqueDetectee { .. }) {
            detected += 1;
        }
    }

    let elapsed_total = start.elapsed();
    let avg_us = total_duration.as_nanos() as f64 / iterations as f64 / 1_000.0;
    let detection_rate = (detected as f64 / iterations as f64) * 100.0;

    println!(
        "[bench] label={:<10} flux={} attack={} iters={} detected={}/{} rate={:.2}% avg={:.2}us total={:?}",
        label,
        flux_size,
        attack_size,
        iterations,
        detected,
        iterations,
        detection_rate,
        avg_us,
        elapsed_total
    );
}

fn main() {
    println!("================================================================");
    println!("📊 MINI BENCHMARK — DÉTECTION DE DIVERGENCE OCTET ÉCONOME");
    println!("================================================================");

    let scenarios = [
        ("tiny", 512usize, 1usize),
        ("tiny", 512usize, 4usize),
        ("medium", 2048usize, 8usize),
        ("medium", 2048usize, 16usize),
        ("large", 8192usize, 64usize),
    ];

    for (label, size, attack) in scenarios {
        run_case(label, size, attack, 200);
    }
}
