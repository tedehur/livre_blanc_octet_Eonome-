use octet_econome::{encode_ascii_compact, injecter_attaque, verificateur_noeud_embarque};
use std::time::Instant;

fn main() {
    println!("================================================================");
    println!("🔥 SIMULATION OFFENSIVE : RED TEAM CYBER-ATTACK ON HOSPITAL LAB");
    println!("================================================================");

    let message = "DOSSIER PATIENT OK ";
    let mut flux_original_sain = encode_ascii_compact(message);
    let mut reseau_alpha_expose = flux_original_sain.clone();
    let reseau_beta_isole = flux_original_sain.clone();

    println!("[+] Réseaux initialisés. État : NOMINAL.");
    println!("[+] Données en transit : {:?}", message);

    println!("\n[!] ALERTE : tentative d’altération du flux Alpha.");
    let chrono_attaque = Instant::now();

    injecter_attaque(&mut reseau_alpha_expose, 100, 40);

    println!("[+] Attaque exécutée sur le réseau Alpha.");

    let evaluation = verificateur_noeud_embarque(&reseau_alpha_expose, &reseau_beta_isole, 19);
    let temps_execution = chrono_attaque.elapsed();

    match evaluation.statut {
        octet_econome::StatutSysteme::CritiqueAttaqueDetectee { raison, action } => {
            println!("\n🛑 [CYBER-DÉFENSE ACTIVÉE]");
            println!("| Type : {}", raison);
            println!("| Latence matérielle : {:?}", temps_execution);
            println!("| Action : {}", action);
            println!("| Résultat : l’attaquant a été bloqué au bit global {}.", evaluation.bits_traites);
            println!("| Données préservées : \"{}\"", evaluation.donnees_decodees);
            println!("================================================================");
            println!("FÉLICITATIONS : la matrice est saine. Aucune corruption propagée.");
        }
        octet_econome::StatutSysteme::Nominal => {
            println!("❌ ÉCHEC : la divergence n’a pas été détectée.");
        }
    }
}
