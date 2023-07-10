pub mod client {
    use crate::game::GameState;

    #[derive(Clone)]
    pub struct Client {
        url: String,
        client: reqwest::Client,
    }

    impl Client {
        pub fn new(url: String) -> Self {
            Self {
                url: format!("http://{url}"),
                client: reqwest::Client::new(),
            }
        }

        pub async fn check_connection(&self) -> Result<reqwest::Response, reqwest::Error> {
            self.client.get(&self.url).send().await
        }

        pub async fn connect_to_game(
            &self,
            guest_url: String,
        ) -> Result<GameState, reqwest::Error> {
            let url = format!("{}/connect", self.url);
            let res = self.client.post(&url).json(&guest_url).send().await?;
            res.json().await
        }

        pub async fn send_game_state(&self, game_state: &GameState) -> Result<(), reqwest::Error> {
            let url = format!("{}/game", self.url);
            let mut game_state = game_state.clone();
            game_state.player_type.flip();
            self.client.post(&url).json(&game_state).send().await?;
            Ok(())
        }
    }
}

pub mod server {
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
    };

    use axum::{
        extract::State,
        routing::{get, post},
        Json, Router,
    };

    use crate::game::GameState;

    pub struct Server {
        pub url: String,
        pub messages: Mutex<VecDeque<ServerMessage>>,
        pub current_game_state: Arc<Mutex<GameState>>,
    }

    pub enum ServerMessage {
        UpdatedGameState,
        GuestConnected(String),
    }

    impl Server {
        pub fn new(url: String, state: Arc<Mutex<GameState>>) -> Self {
            Self {
                url,
                messages: Mutex::new(VecDeque::new()),
                current_game_state: state,
            }
        }

        pub async fn start(self: Arc<Self>) {
            let app = Router::new()
                .route("/", get(root))
                .route("/connect", post(connect))
                .route("/game", post(game))
                .with_state(self.clone());
            axum::Server::bind(&self.url.parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }

    async fn root() -> &'static str {
        "Working!"
    }

    async fn connect(
        State(server): State<Arc<Server>>,
        Json(guest_url): Json<String>,
    ) -> Json<GameState> {
        server
            .messages
            .lock()
            .unwrap()
            .push_back(ServerMessage::GuestConnected(guest_url));
        let mut game_state = server.current_game_state.lock().unwrap().clone();
        game_state.player_type.flip();
        Json(game_state)
    }

    async fn game(
        State(server): State<Arc<Server>>,
        Json(game_state): Json<GameState>,
    ) -> &'static str {
        *server.current_game_state.lock().unwrap() = game_state;
        server
            .messages
            .lock()
            .unwrap()
            .push_back(ServerMessage::UpdatedGameState);
        "OK"
    }
}
