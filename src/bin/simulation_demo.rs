use octet_econome::{encode_ascii_compact, injecter_attaque, verificateur_noeud_embarque};
use std::time::Instant;

fn main() {
    let message = "DOSSIER PATIENT OK ";
    let mut flux_original_sain = encode_ascii_compact(message);
    let mut reseau_alpha_expose = flux_original_sain.clone();
    let reseau_beta_isole = flux_original_sain.clone();

    println!("[+] Simulation nominale initialise le flux principal et le flux de contrôle.");
    println!("[+] Message : {}", message);

    let chrono = Instant::now();
    injecter_attaque(&mut reseau_alpha_expose, 100, 40);
    let evaluation = verificateur_noeud_embarque(&reseau_alpha_expose, &reseau_beta_isole, 19);
    let elapsed = chrono.elapsed();

    println!("[+] Détection exécutée en {:?}", elapsed);

    match evaluation.statut {
        octet_econome::StatutSysteme::CritiqueAttaqueDetectee { raison, action } => {
            println!("[ALERTE] {}", raison);
            println!("[ACTION] {}", action);
            println!("[RESULTAT] Données préservées : {}", evaluation.donnees_decodees);
        }
        octet_econome::StatutSysteme::Nominal => {
            println!("[OK] Aucun écart détecté.");
        }
    }
}
