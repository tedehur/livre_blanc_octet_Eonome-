import time
import sys

def animer_texte(texte, delai=0.03):
    for char in texte:
        sys.stdout.write(char)
        sys.stdout.flush()
        time.sleep(delai)

# 1. ÉTAPE NOMINALE
print("\033[94m=== PHASE 1 : TRAFIC HOSPITALIER NOMINAL (OCTET-ÉCONOME) ===\033[0m")
flux_sain = ["001010011", "001001111", "1", "001001101", "001000101"] # S, O, [Espace], M, E
donnees   = ["   'S'   ", "   'O'   ", " [Espace]", "   'M'   ", "   'E'   "]

for i in range(len(flux_sain)):
    time.sleep(0.4)
    print(f"[Cycle {i+1}]")
    print(f"  ↳ Réseau Alpha (Fibre) : \033[92m{flux_sain[i]}\033[0m -> Interprétation : {donnees[i]}")
    print(f"  ↳ Réseau Bêta  (Laser) : \033[92m{flux_sain[i]}\033[0m -> Interprétation : {donnees[i]}")
    print("  \033[92m[✓] MATCH PARFAIT - Nœud Neuronal Synchrone\033[0m\n")

time.sleep(1)

# 2. INTRODUCIION DE L'ATTAQUE
print("\n\033[93m=== PHASE 2 : INTRUSION ET TENTATIVE DE CHIFFREMENT (RANSOMWARE) ===\033[0m")
animer_texte("[!] Un ransomware tente de modifier l'écriture à la volée sur le terminal expérimental...\n", 0.02)
time.sleep(0.5)

# 3. L'EFFET FUSIBLE ET LA PULVÉRISATION
print(f"\n[Cycle {len(flux_sain)+1}]")
print("  ↳ Réseau Alpha (Infecté) : \033[91m011010101\033[0m <-- \033[91m[XOR CRYPTO ATTACK]\033[0m")
print("  ↳ Réseau Bêta  (Sain)    : \033[92m001000100\033[0m")
time.sleep(0.8)

print("\n\033[91m🚨 [ALERTE CRITIQUE - DIVERGENCE DÉTECTÉE AU CYCLE D'HORLOGE] 🚨\033[0m")
print("----------------------------------------------------------------")
animer_texte("\033[1;91m[-] Erreur de désynchronisation de structure (Effet décalage 9 bits).\n", 0.01)
animer_texte("[-] ORDRE MATÉRIEL : PULVÉRISATION IMMÉDIATE DU CLONE EXPOSÉ.\n", 0.01)
animer_texte("[-] ACTION : Isolation physique du segment réseau Alpha.\n\033[0m", 0.01)
print("----------------------------------------------------------------")
print("\033[92m[✓] RÉSULTAT : Base de données mère 100% Intacte. Service médical basculé sur Bêta.\033[0m\n")
