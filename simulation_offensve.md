## 5. Simulation Offensive & Résistance aux Attaques

Le projet inclut un banc d'essai (`tests/simulation_attaque.rs`) simulant l'injection d'un payload de chiffrement (Ransomware) directement sur la mémoire tampon du réseau exposé.

Pour exécuter la simulation de crise :
```bash
cargo run --bin simulation_attaque
```

### Sortie attendue dans la console :
```text
================================================================
🔥 SIMULATION OFFENSIVE : RED TEAM CYBER-ATTACK ON HOSPITAL LAB
================================================================
[+] Réseaux Alpha et Bêta initialisés. État : NOMINAL.
[+] Données en transit : "DOSSIER PATIENT OK "

[!] ALERTE : Un ransomware pénètre le terminal d'un médecin.
[!] Tentative de chiffrement brut de la mémoire tampon par l'attaquant...
[+] Attaque exécutée sur le réseau Alpha.

[+] Passage du flux dans le comparateur asymétrique...

🛑 [CYBER-DÉFENSE ACTIVÉE SUCCESSFULLY]
| Type : Divergence physique détectée au bit global 100 !
| Latence matérielle : 420 nanosecondes
| Action matérielle : INTERRUPT_HARDWARE: PULVÉRISATION DU CLONE & ISOLEMENT DU BLOC INFECTÉ
| Résultat : L'attaquant a été bloqué au bit global 100.
| Données préservées intactes : "DOSSIER PA"
================================================================
FÉLICITATIONS : La matrice mère est saine. Zéro fichier chiffré.
```
