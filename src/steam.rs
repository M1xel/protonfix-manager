use std::path::PathBuf;
use steamlocate::SteamDir;

#[derive(Debug, Clone)]
pub struct Game {
    pub name: String,
    pub id: u32,
    pub path: PathBuf,
}

impl Game {
    pub fn prefix_path(&self) -> Option<PathBuf> {
        // If path is .../steamapps/common/GameName
        // prefix is .../steamapps/compatdata/AppID/pfx
        let steamapps = self.path.parent()?.parent()?;
        let prefix = steamapps.join("compatdata").join(self.id.to_string()).join("pfx");
        if prefix.exists() {
            Some(prefix)
        } else {
            None
        }
    }
}

pub struct SteamClient;

impl SteamClient {
    pub fn get_games() -> Result<Vec<Game>, String> {
        let steamdir = SteamDir::locate().map_err(|e| e.to_string())?;
        let mut games = Vec::new();

        for library in steamdir.libraries().map_err(|e| e.to_string())? {
            let library = library.map_err(|e| e.to_string())?;
            let library_path = library.path().to_path_buf();
            for app in library.apps() {
                let app = app.map_err(|e| e.to_string())?;
                if let Some(name) = &app.name {
                    let path = library_path
                        .join("steamapps")
                        .join("common")
                        .join(&app.install_dir);
                    
                    games.push(Game {
                        name: name.clone(),
                        id: app.app_id,
                        path,
                    });
                }
            }
        }

        games.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(games)
    }
}