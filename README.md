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

## Mini benchmark de détection

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
- niveau de robustesse observé.

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

## Comment exécuter, comparer et interpréter les résultats

### 1. Lancer les tests

```bash
cargo test
```

Les tests couvrent les scénarios suivants :

- flux nominal sans corruption,
- attaque par inversion de bits,
- troncature du flux,
- insertion de bits parasites,
- cas limite avec flux vide.

### 2. Lancer la démonstration

```bash
cargo run --bin simulation_demo
```

Cette exécution affiche un scénario de détection d’écart entre le flux principal et le flux de contrôle. En cas de divergence, le programme renvoie un statut de type :

```text
CritiqueAttaqueDetectee { raison, action }
```

Un résultat nominal signifie que le flux est homogène, sans divergence identifiée.

### 3. Lancer le benchmark

```bash
cargo run --bin benchmark_detection
```

Le benchmark compare plusieurs configurations de flux et d’attaque, puis affiche :

- taille du flux,
- taille de l’attaque,
- nombre d’itérations,
- taux de détection,
- temps moyen par itération.

### 4. Interpréter les résultats

Le signal le plus important à observer est le taux de détection :

- valeur proche de 100 % : la divergence est détectée systématiquement,
- valeur plus faible : le mécanisme est sensible au type de corruption ou à la taille du flux,
- temps moyen très faible : bon signe de réactivité,
- temps moyen élevé ou très variable : indice de coût de traitement à surveiller.

En pratique, on regarde surtout :

- la stabilité sur plusieurs tailles de flux,
- la montée du temps de traitement avec la charge,
- le comportement dans les attaques les plus petites et les plus grandes.

### 5. Exemple de lecture

```text
[bench] label=tiny     flux=512 attack=1 iters=200 detected=200/200 rate=100.00% avg=...us
[bench] label=medium   flux=2048 attack=16 iters=200 detected=198/200 rate=99.00% avg=...us
```

Un résultat stable autour de 99–100 % sur plusieurs scénarios est un bon indicateur de robustesse. En revanche, des baisses nettes sur des tailles critiques indiquent un point de vigilance à documenter.

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
