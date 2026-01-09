mod db;
mod state;
mod steam;

use std::sync::Arc;

use anyhow::Result;
use gpui::{
    App, Application, Context, FontWeight, ListAlignment, ListState, MouseButton, Pixels, Window,
    WindowOptions, div, list, prelude::*, rgb,
};
use redb::Database;
use serde::{Deserialize, Serialize};
use steam::{Game, SteamClient};

use crate::{db::AppDb, state::AppStore};

struct GameList {
    games: Vec<Game>,
    list_state: ListState,
}

impl GameList {
    fn new(games: Vec<Game>) -> Self {
        let list_state = ListState::new(games.len(), ListAlignment::Top, Pixels::from(100.0));
        Self { games, list_state }
    }

    /// Renders the application header
    fn render_header() -> impl IntoElement {
        div()
            // Flex layout to center content
            .flex()
            .items_center()
            .justify_center()
            // Padding and Background
            .py_4()
            .bg(rgb(0x252526))
            // Drop shadow for depth
            .shadow_md()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .child("ProtonFix Manager"),
            )
    }

    /// Renders a single game item in the list
    fn render_game_item(game: &Game) -> impl IntoElement {
        let game_click = game.clone();
        div()
            .flex()
            .flex_col()
            // Padding
            .px_4()
            .py_2()
            // Bottom border separator
            .border_b_1()
            .border_color(rgb(0x333333))
            // Hover effect
            .hover(|s| s.bg(rgb(0x2d2d2d)))
            .child(div().font_weight(FontWeight::BOLD).child(game.name.clone()))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xaaaaaa))
                    .child(format!("ID: {}", game.id)),
            )
            // Click handler
            .on_mouse_down(MouseButton::Left, move |_event, _win, _cx| {
                if let Some(prefix) = game_click.prefix_path() {
                    println!("Selected: {} | Prefix: {:?}", game_click.name, prefix);
                } else {
                    println!(
                        "Selected: {} | Prefix not found (likely not run via Proton yet)",
                        game_click.name
                    );
                }
            })
    }
}

impl Render for GameList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = AppStore::get(cx);

        let games = self.games.clone();

        div()
            // Main container layout
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            // Header
            .child(Self::render_header())
            // Game List Area
            .child(
                div()
                    .flex_1()
                    .size_full()
                    .flex()
                    // Sidebar:
                    .child(
                        div().w(state.sidebar_width).h_full().child(
                            list(self.list_state.clone(), move |ix, _win, _cx| {
                                let game = &games[ix];
                                Self::render_game_item(game).into_any_element()
                            })
                            .size_full(),
                        ),
                    )
                    .child(div().w_px().bg(rgb(0x333333)))
                    .child(
                        div()
                            .flex_1()
                            .size_full()
                            .bg(rgb(0x1e1e1e))
                            .child("Details go here"),
                    ),
            )
    }
}

fn main() -> Result<()> {
    let db = Arc::new(AppDb::new("protonfix.redb")?);

    // Initial fetch of games
    let games = SteamClient::get_games().unwrap_or_else(|e| {
        eprintln!("Error fetching games: {}", e);
        vec![]
    });

    Application::new().run(move |cx: &mut App| {
        AppStore::init(cx, db.clone());

        let games = games.clone();
        cx.open_window(WindowOptions::default(), move |_, cx| {
            cx.new(|_| GameList::new(games))
        })
        .unwrap();
    });
    Ok(())
}
