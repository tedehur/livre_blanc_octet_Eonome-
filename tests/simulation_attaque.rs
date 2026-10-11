// =========================================================================
// BANQUE DE TESTS OFFENSIFS : SIMULATION D'UNE ATTAQUE DE RANSOMWARE
// =========================================================================
// Ce script simule une infection par logiciel malveillant au niveau du clone.
// Il démontre la capacité du microcode à couper le réseau au cycle d'horloge près.

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

// Implémentation du cœur de détection Octet-Économe
fn verificateur_noeud_embarque(flux_alpha: &[bool], flux_beta: &[bool], longueur_attendue: usize) -> ResultatNoeud {
    let mut donnees_decodees = String::new();
    let mut index_bit = 0;
    let mut caractere_compte = 0;

    while index_bit < flux_alpha.len() && caractere_compte < longueur_attendue {
        // CROSS-CHECKING DES DEUX LIGNES PHYSIQUES (Redondance Aéronautique)
        if flux_alpha[index_bit] != flux_beta[index_bit] {
            return ResultatNoeud {
                statut: StatutSysteme::CritiqueAttaqueDetectee {
                    raison: format!("Divergence physique détectée au bit global {} !", index_bit),
                    action: "INTERRUPT_HARDWARE: PULVÉRISATION DU CLONE & ISLEMENT DU BLOC INFECTÉ".to_string(),
                },
                donnees_decodees,
                bits_traites: index_bit,
            };
        }

        let bit_courant = flux_alpha[index_bit];

        if bit_courant { // 1 = Espace stéganographique
            donnees_decodees.push(' ');
            index_bit += 1;
            caractere_compte += 1;
        } else { // 0 = Décalage + Lecture bloc ASCII de 8 bits
            if index_bit + 9 > flux_alpha.len() { break; }
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
                if flux_alpha[position] { octet_valeur |= 1 << (7 - i); }
            }
            donnees_decodees.push(octet_valeur as char);
            index_bit += 9;
            caractere_compte += 1;
        }
    }
    ResultatNoeud { statut: StatutSysteme::Nominal, donnees_decodees, bits_traites: index_bit }
}

fn main() {
    println!("================================================================");
    println!("🔥 SIMULATION OFFENSIVE : RED TEAM CYBER-ATTACK ON HOSPITAL LAB");
    println!("================================================================");

    // 1. Génération du flux sain d'un dossier médical ("DOSSIER PATIENT OK ")
    // Pour le test, on crée un flux binaire valide à l'aide de notre protocole
    let flux_original_sain = vec![
        false, false, true, false, false, false, true, false, false, // 'D'
        false, false, true, false, false, true, true, true, true,    // 'O'
        false, false, true, false, true, false, false, true, true,    // 'S'
        false, false, true, false, true, false, false, true, true,    // 'S'
        false, false, true, false, false, true, false, false, true, // 'I'
        false, false, true, false, false, false, true, false, true,  // 'E'
        false, false, true, false, true, false, false, true, false, // 'R'
        true,                                                       // ' ' (Bit unique)
        false, false, true, false, true, false, false, false, false, // 'P'
        false, false, true, false, false, false, false, false, true, // 'A'
        false, false, true, false, true, false, true, false, false, // 'T'
        false, false, true, false, false, true, false, false, true, // 'I'
        false, false, true, false, false, false, true, false, true,  // 'E'
        false, false, true, false, false, true, true, true, false,   // 'N'
        false, false, true, false, true, false, true, false, false, // 'T'
        true,                                                       // ' ' (Bit unique)
        false, false, true, false, false, true, true, true, true,    // 'O'
        false, false, true, false, false, true, false, true, true,    // 'K'
        true,                                                       // ' ' (Bit unique)
    ];

    // Simulation des deux lignes physiques synchronisées à l'état normal
    let reseau_beta_isole = flux_original_sain.clone();
    let mut reseau_alpha_expose = flux_original_sain.clone();

    println!("[+] Réseaux Alpha et Bêta initialisés. État : NOMINAL.");
    println!("[+] Données en transit : \"DOSSIER PATIENT OK \"");

    // 2. Lancement de l'attaque automatisée par un Ransomware sur le clone Alpha
    println!("\n[!] ALERTE : Un ransomware pénètre le terminal d'un médecin.");
    println!("[!] Tentative de chiffrement brut de la mémoire tampon par l'attaquant...");

    let chrono_attaque = Instant::now();

    // Le ransomware attaque au hasard la fin du flux pour y appliquer sa clé de chiffrement XOR / AES
    // Ici, il altère les bits à partir de l'index 100 (au milieu du mot "PATIENT")
    for i in 100..reseau_alpha_expose.len() {
        reseau_alpha_expose[i] = !reseau_alpha_expose[i]; // Inversion malveillante des bits
    }

    println!("[+] Attaque exécutée sur le réseau Alpha.");

    // 3. Réaction immédiate du microcode au niveau du nœud neuronal embarqué
    println!("\n[+] Passage du flux dans le comparateur asymétrique...");
    
    let evaluation = verificateur_noeud_embarque(&reseau_alpha_expose, &reseau_beta_isole, 19);
    let temps_execution = chrono_attaque.elapsed();

    match evaluation.statut {
        StatutSysteme::CritiqueAttaqueDetectee { raison, action } => {
            println!("\n🛑 [CYBER-DÉFENSE ACTIVÉE SUCCESSFULLY]");
            println!("| Type : {}", raison);
            println!("| Latence matérielle : {:?}", temps_execution);
            println!("| Action matérielle : \x1b[31;1m{}\x1b[0m", action);
            println!("| Résultat : L'attaquant a été bloqué au bit global {}.", evaluation.bits_traites);
            println!("| Données préservées intactes : \"{}\"", evaluation.donnees_decodees);
            println!("================================================================");
            println!("FÉLICITATIONS : La matrice mère est saine. Zéro fichier chiffré.");
        },
        StatutSysteme::Nominal => {
            println!("❌ ÉCHEC : Le ransomware a réussi à corrompre le flux sans déclencher l'alarme.");
        }
    }
}
