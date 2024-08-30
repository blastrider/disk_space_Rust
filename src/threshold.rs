pub struct Thresholds {
    pub ok: f64,
    pub warning: f64,
    pub critical: f64,
}

impl Thresholds {
    pub fn new(ok: f64, warning: f64, critical: f64) -> Self {
        Self {
            ok,
            warning,
            critical,
        }
    }

    pub fn check(&self, free_percentage: f64) -> (&'static str, &'static str) {
        match free_percentage {
            p if p > self.ok => ("OK", "\x1b[32m"),                // Vert
            p if p > self.warning => ("À surveiller", "\x1b[33m"), // Jaune
            p if p > self.critical => ("Attention", "\x1b[31m"),   // Rouge
            _ => ("Critique - Supprimez des fichiers", "\x1b[41m"), // Rouge avec fond rouge
        }
    }
}
