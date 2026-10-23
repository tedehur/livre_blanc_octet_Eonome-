# Octet Économe

> Prototype de transmission compacte avec redondance et détection de divergence.

[![License: MIT](https://img.shields.io/badge/License-MIT-green)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Prototype-blue)](#)

## Résumé

Octet Économe est un prototype de protocole de transmission inspiré par les principes suivants :

- optimisation du format de transport pour les caractères fréquents,
- ajout d’un signal de contrôle compact,
- comparaison de deux flux indépendants,
- détection rapide de divergence pour couper la propagation d’une corruption.

Le dépôt sert de base de démonstration technique et de test de concept. Il ne prétend pas à une solution de production critique, ni à une validation formelle de sécurité.

## Cadrage scientifique

Le projet est utile comme prototype de recherche et pour l’illustration de mécanismes de redondance structurée, notamment :

- comparaison bit-à-bit de flux,
- détection d’altération locale ou globale,
- réaction immédiate au premier écart,
- démonstration d’un mécanisme de sécurité par divergence.

## Structure du dépôt

```text
.
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── lib.rs
│   └── bin/
│       ├── simulation_demo.rs
│       └── benchmark_detection.rs
├── tests/
│   └── offensive_detection.rs
├── docs/
│   ├── design_notes.md
│   └── roadmap_maturite.md
├── scripts/
│   └── visuel.py
└── legacy/
    └── simulation_offensve.md
```

## Modèle de fonctionnement

Le prototype fonctionne selon un principe simple :

- un flux principal est encodé de manière compacte,
- un flux de contrôle est comparé au flux principal,
- tout écart entre les deux flux est considéré comme une anomalie potentielle,
- le système réagit au premier point de divergence détecté.

## Exemple de codage

```text
[1] -> interprété comme un espace
[0][01000001] -> bit de contrôle + octet ASCII 'A'
```

## Tests disponibles

Le dépôt contient désormais une base de tests Rust exploitable pour tester les cas suivants :

- flux nominal sans corruption,
- divergence locale sur le flux principal,
- interruption/troncature de flux,
- insertion de bits parasites,
- détection d’anomalie sur un flux de comparaison.

## Benchmark de détection

Un mini benchmark est inclus pour mesurer la costabilité de la détection sur plusieurs tailles de flux et niveaux d’attaque.

```bash
cargo run --bin benchmark_detection
```

Le benchmark exécute des variations sur :

- taille du flux : 512, 2048, 8192 bits,
- amplitude de l’attaque : 1, 4, 8, 16, 64 bits,
- nombre d’itérations : 200+ reproductions.

Il affiche :

- temps total d’exécution,
- temps moyen par essai,
- taux de détection,
- niveau de robustesse observée.

## Feuille de route de maturité

Le projet suit une progression utile et honnête :

1. Prototype fonctionnel
   - encodage compact,
   - comparaison de flux,
   - détection binaire par divergence.
2. Formalisation du protocole
   - règles de parsing,
   - gestion d’erreurs,
   - cas de troncature et d’insertion.
3. Robustesse offensive
   - tests adversariaux,
   - mesures de détection,
   - scénarios de corruption réalistes.
4. Instrumentation de recherche
   - benchmarks,
   - journalisation des détections,
   - métriques de latence.
5. Maturité logicielle
   - CI,
   - documentation mécanique,
   - API plus stable,
   - versionnement sémantique.

## Exécution locale

```bash
cargo test
cargo run --bin simulation_demo
cargo run --bin benchmark_detection
```

## Limites

La démonstration est un prototype de recherche. Elle n’a pas vocation à remplacer :

- un système critique en production,
- une validation formelle de sécurité,
- une implémentation de protocole certifiée.

## Licence

MIT.
