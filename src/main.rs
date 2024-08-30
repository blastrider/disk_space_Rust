mod compatibility;
mod filesystem;
mod threshold;

use compatibility::is_supported_platform;
use filesystem::{get_disk_usage, is_special_filesystem};
use threshold::Thresholds;

fn check_disk_space(path: &str, thresholds: &Thresholds) {
    if !is_supported_platform() {
        eprintln!("Plateforme non prise en charge pour le chemin : {}", path);
        return;
    }

    if is_special_filesystem(path) {
        println!("Système de fichiers spécial ignoré : {}", path);
        return;
    }

    match get_disk_usage(path) {
        Ok((free, total)) => {
            let free_percentage = (free as f64 / total as f64) * 100.0;
            let free_gb = free as f64 / (1024.0 * 1024.0 * 1024.0);
            let total_gb = total as f64 / (1024.0 * 1024.0 * 1024.0);

            let (status, color) = thresholds.check(free_percentage);

            println!("Disque: {}", path);
            println!(
                "{}Statut: {} ({:.2}% libre)\x1b[0m",
                color, status, free_percentage
            );
            println!("Total: {:.2} Go, Libre: {:.2} Go\n", total_gb, free_gb);
        }
        Err(e) => eprintln!("Erreur lors de la vérification du disque {}: {}", path, e),
    }
}

fn main() {
    // Définition des chemins par défaut en fonction de l'OS
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let paths = vec!["/"];

    #[cfg(target_os = "windows")]
    let paths = vec!["C:\\"];

    #[cfg(target_os = "macos")]
    let paths = vec!["/"];

    let thresholds = Thresholds::new(80.0, 75.0, 70.0);

    for path in paths {
        check_disk_space(path, &thresholds);
    }
}
