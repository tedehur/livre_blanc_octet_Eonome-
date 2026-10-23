#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatutSysteme {
    Nominal,
    CritiqueAttaqueDetectee { raison: String, action: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultatNoeud {
    pub statut: StatutSysteme,
    pub donnees_decodees: String,
    pub bits_traites: usize,
}

pub fn encode_ascii_compact(texte: &str) -> Vec<bool> {
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

pub fn verificateur_noeud_embarque(
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

            donnees_decodees.push(octet_valeur as char);
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

pub fn injecter_attaque(flux: &mut [bool], debut: usize, longueur: usize) {
    for i in debut..(debut + longueur).min(flux.len()) {
        flux[i] = !flux[i];
    }
}
