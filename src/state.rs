use crate::db::AppDb;
use gpui::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinePrefix {
    pub name: String,
    pub path: String,
    pub wine_version: String,
    pub proton: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub prefixes: Vec<WinePrefix>,
    pub sidebar_width: f32,
    pub last_selected: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            prefixes: vec![],
            sidebar_width: 250.0,
            last_selected: None,
        }
    }
}

pub struct AppStore {
    state: Arc<RwLock<AppState>>,
    db: Arc<AppDb>,
}

impl Global for AppStore {}

impl AppStore {
    pub fn init(cx: &mut App, db: Arc<AppDb>) {
        let state = db.load("app_state").ok().flatten().unwrap_or_default();

        cx.set_global(Self {
            state: Arc::new(RwLock::new(state)),
            db,
        });
    }

    // Simple update helper
    pub fn update(cx: &mut App, f: impl FnOnce(&mut AppState)) {
        let store = cx.global::<Self>();

        {
            let mut state = store.state.write().unwrap();
            f(&mut state);
            let _ = store.db.save("app_state", &*state);
        }

        cx.refresh_windows();
    }

    pub fn get(cx: &App) -> AppState {
        let store = cx.global::<Self>();
        store.state.read().unwrap().clone()
    }
}
