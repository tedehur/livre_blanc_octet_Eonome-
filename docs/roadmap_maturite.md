# Feuille de route de maturité

## 1. Objectif

Le projet Octet Économe est aujourd’hui un prototype fonctionnel de démonstration. Le but de la feuille de route est de clarifier ce qu’il faut faire pour passer d’un artefact de recherche à une base plus solide et plus exploitable.

## 2. Niveau actuel

### État constaté

- encodage compact prototype,
- comparaison de flux en place,
- détection de divergence opérationnelle,
- tests de base en Rust,
- benchmark minimal ajouté,
- documentation orientée GitHub en cours d’amélioration.

### Limites actuelles

- pas de validation de sécurité formelle,
- pas de modèle de menace détaillé,
- pas de benchmark exhaustif,
- pas de CI / automatisation de test,
- pas de version stable de l’API publique.

## 3. Cibles de maturité

### Niveau A — prototype valide

- encodage compact cohérent,
- flux principal et flux de contrôle correctement comparés,
- détection d’écart reproductible,
- tests sur cas nominal + corruption + troncature + insertion.

### Niveau B — robustesse mesurée

- benchmark reproductible sur tailles de flux variées,
- mesures de latence et de détection,
- seuils de détection documentés,
- scénarios adversariaux formalisés.

### Niveau C — cadre de recherche

- documentation de l’architecture,
- portée exacte de la démonstration,
- évaluation empirique de la robustesse,
- séparation entre prototype, démonstration et produit.

### Niveau D — outils d’exploitation

- CI avec cargo test,
- scripts d’analyse et d’export,
- tests multi-plateformes,
- documentation de contribution claire.

## 4. Plan d’action recommandé

### Horizon 1 (court terme)

- stabiliser la base Rust,
- enrichir les cas de tests,
- documenter les limites internes,
- mesurer la détection sur plusieurs tailles de flux.

### Horizon 2 (moyen terme)

- formaliser le protocole,
- ajouter des scénarios d’attaque plus réalistes,
- améliorer les métriques et la journalisation,
- créer un mini rapport de benchmark.

### Horizon 3 (long terme)

- définir un standard de sécurité interne,
- préparer une API propre,
- dimensionner la conception pour une version plus structurée,
- décider du statut exact du projet : prototype, démonstration ou produit.

## 5. Critères de succès

Le projet est mature au sens utile du terme quand il atteint les critères suivants :

- les cas de corruption sont bien testés,
- les performances de détection sont mesurées,
- les limitations sont exposées sans ambiguïté,
- les contributeurs savent ce que le projet est et ce qu’il n’est pas.
