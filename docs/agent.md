# Agent Instructions: R6 Siege Hydra Tracker Wrapper

## 🎯 Contexte et Problème
L'utilisateur souhaite traquer son temps de jeu sur **Rainbow Six Siege** via **Hydra Launcher**. 
Actuellement, le tracking échoue car le lancement implique une chaîne de processus : l'exécutable initial s'ouvre, puis se ferme pour laisser place à Ubisoft Connect, BattlEye, et d'éventuelles mises à jour. Hydra détecte la fermeture de l'exécutable initial comme la fin de la session et arrête le chronomètre.
Les solutions basées sur des scripts Batch (.bat) ont échoué car elles ne gèrent pas bien les temps d'attente longs et instables (synchronisation cloud, mises à jour Ubisoft).

## 🚀 Objectif
Générer et configurer un "wrapper" natif en **Rust** qui servira d'exécutable cible pour Hydra. 

Ce programme doit répondre aux critères suivants :
1. **Invisibilité totale** : Fonctionner en arrière-plan sans ouvrir d'invite de commande (fenêtre CMD).
2. **Lancement intelligent** : Démarrer le jeu via le protocole Ubisoft (`uplay://launch/635/0` pour Ubisoft Store ou `uplay://launch/1843/0` pour Steam) pour s'assurer que le launcher officiel gère la connexion, les syncs et BattlEye.
3. **Patience (Timeout long)** : Attendre l'apparition du processus final (`RainbowSix.exe` ou `RainbowSix_Vulkan.exe`) avec une tolérance allant jusqu'à 5 minutes pour couvrir les temps de chargement/màj.
4. **Maintien de session** : Rester actif (maintenir son propre PID) tant que le processus du jeu tourne.
5. **Fermeture propre** : Se terminer automatiquement dès que le processus du jeu n'est plus détecté.

## 🛠️ Instructions pour le LLM / IDE
En tant qu'assistant de code, tu dois guider l'utilisateur dans la création de ce projet. Voici les étapes à générer et à valider :

### 1. Configuration du projet
- Créer un projet Rust : `cargo new r6_tracker`
- Ajouter la crate nécessaire pour lire les processus système : `cargo add sysinfo`

### 2. Implémentation du code (src/main.rs)
- Ajouter l'attribut `#![windows_subsystem = "windows"]` en haut du fichier pour désactiver l'interface console Windows.
- Utiliser `std::process::Command` pour exécuter : `cmd /C start uplay://launch/635/0`.
- Initialiser `sysinfo::System::new_all()`.
- **Phase 1 (Recherche)** : Créer une boucle (ex: 300 itérations avec 1 seconde de `thread::sleep`) qui fait un `sys.refresh_processes()` et cherche `"RainbowSix.exe"` ou `"RainbowSix_Vulkan.exe"`. Si trouvé, sortir de la boucle.
- **Phase 2 (Surveillance)** : Créer une boucle infinie qui vérifie toutes les 5 secondes si le jeu tourne toujours. Si le processus n'est plus trouvé, faire un `break` pour terminer le programme.

### 3. Compilation et Déploiement
- Demander à l'utilisateur de compiler avec `cargo build --release`.
- Lui expliquer de récupérer l'exécutable dans `target/release/r6_tracker.exe`.
- Lui rappeler de configurer Hydra Launcher pour pointer vers ce fichier `.exe` généré.

## ⚠️ Points d'attention
- La consommation CPU doit être quasi-nulle (les `thread::sleep` sont obligatoires dans les boucles).
- Le code doit tourner sous Windows.
- Assurer une gestion minimale des erreurs (par exemple, si la commande de lancement échoue).
