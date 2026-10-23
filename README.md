# Octet Économe

> Prototype de transmission compacte, redondance structurelle et détection rapide de divergence.

[![License: MIT](https://img.shields.io/badge/License-MIT-green)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Prototype-blue)](#)

## Vue d’ensemble

Octet Économe est un prototype de protocole de transmission conçu comme une démonstration de concept sur la redondance structurée et la détection de divergence.

L’idée centrale est simple :

- compresser le format de transport des caractères les plus fréquents,
- conserver un signal de contrôle compact,
- comparer deux flux indépendants,
- réagir immédiatement au premier écart détecté.

Le projet ne prétend pas à une solution de production critique. Il vise plutôt à fournir un cadre reproductible d’expérimentation, de démonstration technique et de discussion scientifique sur les mécanismes de détection de corruption et d’intégrité de flux.

## Pourquoi ce projet existe

Les systèmes critiques doivent souvent choisir entre :

- transmission compacte,
- redondance de contrôle,
- détection rapide des altérations,
- réaction immédiate à l’anomalie.

Octet Économe explore cette combinaison sous une forme minimale et facilement testable.

## Architecture de la démonstration

### 1. Encodage compact

Le protocole associe un bit de contrôle à un bloc de données standard. Les caractères fréquents peuvent être représentés de manière plus compacte, tandis que les caractères moins fréquents restent encodés selon une convention explicite.

Exemple :

```text
[1] -> interprété comme un espace
[0][01000001] -> bit de contrôle + octet ASCII 'A'
```

### 2. Redondance de flux

Le système compare :

- un flux principal,
- un flux de contrôle ou miroir.

Le cœur du mécanisme repose sur une comparaison bit-à-bit. Toute divergence est interprétée comme une anomalie possible.

### 3. Détection de divergence

Le moteur de validation analyse les deux flux et déclenche un état de détection dès qu’un écart apparaît. Cela permet d’isoler l’origine de la corruption et d’empêcher sa propagation.

## Ce que le dépôt contient

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

## Démonstration fonctionnelle

Le dépôt contient une simulation de flux avec injection d’attaque, ainsi qu’un mini benchmark de performances.

### Scénarios couverts

- flux nominal sans corruption,
- inversion de bits sur le flux principal,
- troncature de signal,
- insertion de bits parasites,
- cas limite avec flux vide.

## Exécution rapide

### Tests

```bash
cargo test
```

### Démonstration

```bash
cargo run --bin simulation_demo
```

### Benchmark

```bash
cargo run --bin benchmark_detection
```

## Comment lire les résultats

### État nominal

Le programme renvoie un statut de type :

```text
Nominal
```

Cela indique qu’aucune divergence n’a été détectée entre les deux flux.

### État d’attaque détectée

Le moteur peut renvoyer un statut de type :

```text
CritiqueAttaqueDetectee { raison, action }
```

Ce signal signifie qu’une divergence a été observée et qu’une action de protection a dû être déclenchée.

### Benchmark

Le mini benchmark affiche :

- taille du flux,
- taille de l’attaque,
- nombre d’itérations,
- taux de détection,
- temps moyen par essai.

On interprète ces résultats en regardant surtout :

- stabilité de la détection sur plusieurs tailles de flux,
- taux de détection proche de 100 % sur des scénarios représentatifs,
- coût en temps de traitement acceptable pour la taille de charge considérée.

## Feuille de route de maturité

Le projet suit une progression claire et honnête :

1. Prototype fonctionnel
2. Formalisation du protocole
3. Robustesse offensive
4. Instrumentation de recherche
5. Maturité logicielle

Pour plus de détails, voir :

- `docs/roadmap_maturite.md`
- `docs/design_notes.md`

## Limites et cadrage

Le projet est un prototype de recherche et de démonstration technique. Il ne doit pas être interprété comme :

- une solution de sécurité certifiée,
- un mécanisme de production critique,
- une preuve formelle de robustesse adversariale.

Cette limite est volontairement explicite afin de garder une base claire, reproductible et critique.

## Licence

MIT.

## Contribution

Les améliorations les plus utiles concernent :

- formalisation du protocole,
- scénarios d’attaque plus réalistes,
- instrumentations de benchmark,
- documentation d’architecture et de contribution.

---

Ce dépôt est conçu comme une base technique reproductible et lisible pour une démonstration d’intégrité de flux par redondance structurée.
