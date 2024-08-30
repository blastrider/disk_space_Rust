# Disk Space Monitor

## Description

Ce projet est un programme en Rust conçu pour monitorer l'espace disque sur plusieurs systèmes d'exploitation, notamment Linux, Windows, macOS et FreeBSD. Le programme analyse l'espace libre sur les disques et fournit des recommandations basées sur des seuils personnalisables, tout en utilisant des codes couleurs pour indiquer visuellement les niveaux de criticité.

## Fonctionnalités

- **Portabilité** : Compatible avec Linux, Windows, macOS, et FreeBSD.
- **Codes couleurs** : Indique l'état de l'espace disque avec des couleurs :
  - Vert (`OK`) : Plus de 50% de l'espace libre.
  - Jaune (`À surveiller`) : Entre 30% et 50% de l'espace libre.
  - Rouge (`Attention`) : Entre 10% et 30% de l'espace libre.
  - Rouge sur fond rouge (`Critique - Supprimez des fichiers`) : Moins de 10% de l'espace libre.
- **Séparé en modules** :
  - **filesystem** : Gère la récupération des informations sur les systèmes de fichiers.
  - **compatibility** : Vérifie la compatibilité avec le système d'exploitation.
  - **threshold** : Définit et vérifie les seuils avec les codes couleurs.

## Installation

1. Assurez-vous d'avoir installé Rust sur votre machine. Si ce n'est pas le cas, installez-le via [rustup](https://rustup.rs/).

2. Clonez ce dépôt :

   ```bash
   git clone https://github.com/votre-utilisateur/disk-space-monitor.git
   cd disk-space-monitor
   ```

3. Ajoutez les dépendances nécessaires dans votre fichier `Cargo.toml` :

   ```toml
   [dependencies]
   nix = { version = "0.29.0", features = ["fs"] }
   winapi = { version = "0.3", features = ["fileapi", "winnt"] }
   ```

4. Compilez le projet :

   ```bash
   cargo build --release
   ```

5. Exécutez le programme :

   ```bash
   cargo run --release
   ```

## Utilisation

Le programme est conçu pour être utilisé en ligne de commande sans paramètres, il faut spécifier cles chemis dans le code, impémentaion d'un fichier de configuration à prévoir. Par défaut, il vérifiera l'espace disque sur les chemins suivants :

- **Linux/FreeBSD** : `/`
- **Windows** : `C:\`
- **macOS** : `/`

Les résultats affichent le statut de l'espace disque avec des couleurs pour une lecture facile.

## Personnalisation

Vous pouvez ajuster les seuils de criticité dans le fichier `threshold.rs` :

```rust
let thresholds = Thresholds::new(50.0, 30.0, 10.0);
```

Modifiez les valeurs `50.0`, `30.0`, et `10.0` pour changer les seuils correspondant aux statuts `OK`, `Warning`, et `Critical`.

## Contribution

Les contributions sont les bienvenues ! Veuillez soumettre une pull request ou ouvrir une issue pour discuter des changements que vous proposez.

## Licence

Ce projet est sous licence MIT. Consultez le fichier `LICENSE` pour plus d'informations.
