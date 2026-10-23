# Écosystème Octet-Économe

> Prototype de protocole de transmission compact et redondé, orienté recherche et démonstration de concept.

[![License: MIT](https://img.shields.io/badge/License-MIT-green)](https://opensource.org/licenses/MIT)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![Status: Prototype](https://img.shields.io/badge/Status-Prototype-blue)](#)

## Résumé

Ce dépôt documente et illustre un protocole expérimental de transmission structurée en deux dimensions :

- un codage compact inspiré du binaire asymétrique,
- une redondance de flux sur deux voies physiques,
- un mécanisme de détection basé sur la divergence de signaux.

L’objectif n’est pas de prétendre à un produit de sécurité prêt pour la production, mais de proposer une architecture de démonstration claire, reproductible et facilement discutée.

Le projet se place dans la catégorie des prototypes de recherche : il vise à explorer la combinaison entre:

- compression légère du format de transport,
- redondance de transmission,
- comparaison de flux,
- réaction immédiate face à une divergence anormale.

## Ce que le projet vise à démontrer

1. Réduire la redondance structurelle d’un flux textuel simple.
2. Faire coexister un encodage compact et une détection de corruption rapide.
3. Utiliser deux voies de transmission indépendantes pour comparer les signaux.
4. Réagir au premier écart détecté, afin de limiter l’ampleur de l’intrusion.

## Limites et cadrage

Le code présent est un prototype de démonstration.

Il ne prétend pas à:

- une validation sécurité formelle,
- un système de production critique,
- une implémentation complètement optimisée,
- une preuve de résilience absolue contre des attaques sophistiquées.

Le bon cadre de lecture est donc celui d’un artefact de recherche et de pédagogie : un point de départ pour mesurer des idées, les discuter et les itérer.

---

## 1. Protocole Octet-Économe : principe

Le protocole repose sur une idée simple : distinguer,

- les symboles les plus fréquents (comme l’espace),
- des blocs de données standard codés sur 8 bits.

L’idée est de représenter l’espace par un bit unique de type signal, puis d’envoyer les autres caractères selon une convention explicite.

### Exemple

```text
[1] -> interprété comme un espace
[0][01000001] -> bit de contrôle + octet ASCII 'A'
```

Cela laisse une structure compacte, mais qui reste lisible et contrôlable par un parseur simple.

---

## 2. Redondance et comparaison de flux

Le mode de démonstration présente deux flux distincts :

- Flux Alpha : voie principale
- Flux Bêta : voie de contrôle et de comparaison

Le cœur du mécanisme est la comparaison bit à bit. Si les deux flux divergent, le système déclenche une réaction immédiate.

### Hypothèse de travail

En l’absence de corruption, les deux flux doivent rester concordants. Une divergence est interprétée comme un signal d’anomalie, et non comme une simple différence de rendu.

---

## 3. Preuve de concept Rust

Le code suivant présente le cœur de détection du prototype. Il ne s’agit pas d’un produit fini, mais d’un point d’entrée exploitable pour une simulation ou une validation d’architecture.

```rust
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

fn verificateur_noeud_embarque(
    flux_alpha: &[bool],
    flux_beta: &[bool],
    longueur_attendue: usize,
) -> ResultatNoeud {
    let mut donnees_decodees = String::new();
    let mut index_bit = 0;
    let mut caractere_compte = 0;

    while index_bit < flux_alpha.len() && caractere_compte < longueur_attendue {
        if flux_alpha[index_bit] != flux_beta[index_bit] {
            return ResultatNoeud {
                statut: StatutSysteme::CritiqueAttaqueDetectee {
                    raison: format!("Divergence physique détectée au bit global {} !", index_bit),
                    action: "PULVÉRISATION INSTANTANÉE DU CLONE ET ISOLEMENT DU SEGMENT".to_string(),
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
            if index_bit + 9 > flux_alpha.len() {
                break;
            }

            let mut octet_valeur: u8 = 0;

            for i in 0..8 {
                let position = index_bit + 1 + i;

                if flux_alpha[position] != flux_beta[position] {
                    return ResultatNoeud {
                        statut: StatutSysteme::CritiqueAttaqueDetectee {
                            raison: format!("Sabotage détecté dans le bloc ASCII à la position {}", position),
                            action: "PULVÉRISATION DU CLONE".to_string(),
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
```

---

## 4. Simulation offensive

Le dépôt contient une simulation d’attaque qui modifie un flux sur une voie de transmission et vérifie que le système le détecte.

Le script de démonstration est placé dans :

- `tests/simulation_attaque.rs`
- `visuel.py`

L’objectif est de vérifier visuellement et par code qu’un écart de flux produit bien une alerte structurée, et qu’une isolation est déclenchée avant qu’une corruption ne se propage.

### Ce qu’il faut retenir

- La simulation est démonstrative.
- Elle ne valide pas un système critique en production.
- Elle sert à faire une preuve de concept reproductible.

---

## 5. Évolution proposée

Le projet peut évoluer dans plusieurs directions, sans perdre son cadrage de prototype.

### Axe 1 — formalisation du protocole

Clarifier précisément :

- structure du bit de contrôle,
- convention d’encodage des octets,
- règle de parsing,
- gestion des erreurs,
- cas de troncature, bits manquants, flux incohérents.

### Axe 2 — sécurité offensive

Ajouter des tests de robustesse :

- bit-flip aléatoire,
- troncature de flux,
- insertion de bits parasites,
- divergence localisée,
- attaque sur les deux voies simultanément.

### Axe 3 — instrumentations de mesure

Ajouter des métriques de diagnostic :

- délai de détection,
- bit d’alarme,
- couverture de détection,
- taux de faux positifs,
- coût de traitement.

### Axe 4 — documentation de référence

Les sections de documentation doivent distinguer clairement :

- démonstration,
- hypothèse de travail,
- limite de validité,
- piste d’évolution technique.

---

## 6. Structure du dépôt

```text
.
├── README.md
├── LICENSE
├── simulation_offensve.md
├��─ visuel.py
├── tests/
│   └── simulation_attaque.rs
└── .gitignore
```

---

## 7. Conclusion

Le dépôt représente un prototype de transmission compact avec redondance de contrôle et donc un terrain de laboratoire utile pour réfléchir à la sécurité des flux, à la détection de divergence et à la robustesse structurée.

Il est utile comme démonstration technique, mais il doit rester explicitement cadré comme projet de recherche et non comme solution de sécurité finale.

---

## 8. Licence

Ce projet est distribué sous licence MIT.

---

## 9. Contribution

Les améliorations souhaitées portent notamment sur :

- la formalisation du protocole,
- la stabilité des scripts de simulation,
- la clarté des documents de démonstration,
- la séparation entre prototype, démonstration et produit.

Les contributions sont les bienvenues dans le cadre d’une démarche claire et reproductible.

"""

This repository contains a compact-code / redundant-flow prototype. The original README was over-claiming and too product-like. This revision keeps the idea, clarifies the limits, and frames the project as an experimental research prototype.

"""



























































