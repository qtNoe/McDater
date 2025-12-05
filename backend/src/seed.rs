use sqlx::MySqlPool;
use std::{fs, path::Path};

// Pfad relativ zum Cargo.toml (Projekt-Root)
const SEEDS_DIR: &str = "seeds"; 

pub async fn run_seeding(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running seed data files...");
    
    // 1. Pfad zum Seed-Verzeichnis erstellen
    let seed_path = Path::new(SEEDS_DIR);

    // 2. Alle Seed-Dateien finden und sortieren
    let mut entries: Vec<_> = fs::read_dir(seed_path)?
        .filter_map(|res| res.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "sql"))
        .collect();

    // Sortiere nach Dateinamen (01_..., 02_...) um die Ausführungsreihenfolge zu gewährleisten
    entries.sort_by_key(|entry| entry.file_name());

    // 3. Jede Datei ausführen
    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        let sql = fs::read_to_string(&path)?;

        println!("-> Executing seed file: {}", file_name);

        // SQL-Befehle ausführen
        match sqlx::query(&sql).execute(pool).await {
            Ok(_) => println!("   ✅ Success."),
            Err(e) => {
                // Bei einem Fehler abbrechen oder protokollieren (je nach Wunsch)
                eprintln!("   ❌ Error executing {}: {}", file_name, e);
                // WICHTIG: Wenn der Seed fehlschlägt, willst du wahrscheinlich abbrechen
                return Err(Box::new(e)); 
            }
        }
    }

    println!("Seed data execution complete.");
    Ok(())
}