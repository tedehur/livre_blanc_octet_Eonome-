// =========================================================================
// BANQUE DE TESTS OFFENSIFS : SIMULATION D'UNE ATTAQUE DIVERGENTE
// =========================================================================
// Ce script se veut une démonstration reproductible du comportement d'un
// système de comparaison de flux avec redondance. Il ne s'agit pas d'un
// système de production : c'est un prototype de validation de concept.

use std::time::Instant;

#[derive(Debug, PartialEq)]
enum StatutSysteme {
    Nominal,
    CritiqueAttaqueDetectee { raison: String, action: String },
}

struct ResultatNoeud {
    statut: StatutSysteme,
    donnees_decodees: String,
    bits_traites: usize,
}

fn encode_ascii_compact(texte: &str) -> Vec<bool> {
    let mut flux = Vec::new();

    for ch in texte.chars() {
        if ch == ' ' {
            flux.push(true);
        } else {
            let valeur = ch as u8;
            flux.push(false);
            for bit in (0..8).rev() {
                flux.push(((valeur >> bit) & 1) == 1);
            }
        }
    }

    flux
}

fn verificateur_noeud_embarque(
    flux_alpha: &[bool],
    flux_beta: &[bool],
    longueur_attendue: usize,
) -> ResultatNoeud {
    let mut donnees_decodees = String::new();
    let mut index_bit = 0;
    let mut caractere_compte = 0;

    while index_bit < flux_alpha.len() && caractere_compte < longueur_attendue {
        if index_bit >= flux_beta.len() || flux_alpha[index_bit] != flux_beta[index_bit] {
            return ResultatNoeud {
                statut: StatutSysteme::CritiqueAttaqueDetectee {
                    raison: format!("Divergence physique détectée au bit global {} !", index_bit),
                    action: "INTERRUPT_HARDWARE: PULVÉRISATION DU CLONE & ISOLEMENT DU SEGMENT".to_string(),
                },
                donnees_decodees,
                bits_traites: index_bit,
            };
        }

        let bit_courant = flux_alpha[index_bit];

        if bit_courant {
            donnees_decodees.push(' ');
            index_bit += 1;
            caractere_compte += 1;
        } else {
            if index_bit + 9 > flux_alpha.len() || index_bit + 9 > flux_beta.len() {
                break;
            }

            let mut octet_valeur: u8 = 0;
            for i in 0..8 {
                let position = index_bit + 1 + i;

                if flux_alpha[position] != flux_beta[position] {
                    return ResultatNoeud {
                        statut: StatutSysteme::CritiqueAttaqueDetectee {
                            raison: format!("Sabotage détecté dans le bloc ASCII à l'index {}", position),
                            action: "INTERRUPT_HARDWARE: REJET ÉCRITURE".to_string(),
                        },
                        donnees_decodees,
                        bits_traites: position,
                    };
                }

                if flux_alpha[position] {
                    octet_valeur |= 1 << (7 - i);
                }
            }

            let caractere = octet_valeur as char;
            donnees_decodees.push(caractere);
            index_bit += 9;
            caractere_compte += 1;
        }
    }

    ResultatNoeud {
        statut: StatutSysteme::Nominal,
        donnees_decodees,
        bits_traites: index_bit,
    }
}

fn injecter_attaque(flux: &mut [bool], debut: usize, longueur: usize) {
    for i in debut..(debut + longueur).min(flux.len()) {
        flux[i] = !flux[i];
    }
}

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

    // Injection d'une déviation nette sur le flux Alpha.
    // La plage choisie est volontairement cohérente avec l’exemple de démonstration.
    injecter_attaque(&mut reseau_alpha_expose, 100, 40);

    println!("[+] Attaque exécutée sur le réseau Alpha.");

    let evaluation = verificateur_noeud_embarque(&reseau_alpha_expose, &reseau_beta_isole, 19);
    let temps_execution = chrono_attaque.elapsed();

    match evaluation.statut {
        StatutSysteme::CritiqueAttaqueDetectee { raison, action } => {
            println!("\n🛑 [CYBER-DÉFENSE ACTIVÉE]");
            println!("| Type : {}", raison);
            println!("| Latence matérielle : {:?}", temps_execution);
            println!("| Action : {}", action);
            println!("| Résultat : l’attaquant a été bloqué au bit global {}.", evaluation.bits_traites);
            println!("| Données préservées : \"{}\"", evaluation.donnees_decodees);
            println!("================================================================");
            println!("FÉLICITATIONS : la matrice est saine. Aucune corruption propagée.");
        }
        StatutSysteme::Nominal => {
            println!("❌ ÉCHEC : la divergence n’a pas été détectée.");
        }
    }
}
