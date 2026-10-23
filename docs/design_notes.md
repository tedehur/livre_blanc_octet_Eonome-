# Design notes

Ce document décrit les grands axes de l’évolution technique du prototype.

## Objectif

Le but de cette base est de rendre le projet plus reproductible et plus simple à tester :

- encodage compact,
- comparaison de flux,
- détection de divergence,
- démonstration d’attaque et de réaction.

## Pistes de développement

### 1. Formalisation du protocole

- définir précisément la structure du bit de contrôle,
- fixer les règles de parsing,
- définir le comportement en cas de flux tronqué,
- clarifier les motifs de détection acceptés.

### 2. Robustesse offensive

Ajouter des cas d’attaque réalistes :

- troncature de bout de flux,
- insertion de bits parasites,
- mutation d’un bloc central,
- divergence sur plusieurs sections du signal.

### 3. Mesures de diagnostic

Ajouter des métriques de suivi :

- délai de détection,
- bit d’alarme,
- nombre de divergences détectées,
- coût de traitement par flux.

### 4. Documentation publique

Le projet est utile comme démonstration technique, mais il doit rester honnête sur son statut :

- prototype,
- démonstration technique,
- point de départ de recherche.
