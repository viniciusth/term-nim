pub mod client {
    use crate::game::GameState;

    pub struct Client {
        url: String,
        client: reqwest::Client,
    }

    impl Client {
        pub fn new(url: String) -> Self {
            Self {
                url,
                client: reqwest::Client::new(),
            }
        }

        pub async fn check_connection(&self) -> bool {
            match self.client.get(&self.url).send().await {
                Ok(r) => r.status().is_success(),
                Err(_) => false,
            }
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
            self.client.post(&url).json(game_state).send().await?;
            Ok(())
        }
    }
}

pub mod server {
    use std::sync::Arc;

    use axum::{
        extract::State,
        routing::{get, post},
        Json, Router,
    };
    use local_ip_address::local_ip;
    use tokio::sync::{mpsc, Mutex};

    use crate::game::GameState;

    pub struct Server {
        url: String,
        sender: mpsc::Sender<ServerMessage>,
        current_game_state: Arc<Mutex<GameState>>,
    }

    pub enum ServerMessage {
        UpdatedGameState,
        GuestConnected(String),
    }

    impl Server {
        pub fn new(sender: mpsc::Sender<ServerMessage>, state: Arc<Mutex<GameState>>) -> Self {
            Self {
                url: format!("{}:40401", local_ip().unwrap()),
                sender,
                current_game_state: state,
            }
        }

        pub async fn start(self) {
            let shared_state = Arc::new(self);
            let app = Router::new()
                .route("/", get(root))
                .route("/connect", post(connect))
                .route("/game", post(game))
                .with_state(shared_state.clone());
            axum::Server::bind(&shared_state.url.parse().unwrap())
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
            .sender
            .send(ServerMessage::GuestConnected(guest_url))
            .await
            .unwrap();
        Json(server.current_game_state.lock().await.clone())
    }

    async fn game(
        State(server): State<Arc<Server>>,
        Json(game_state): Json<GameState>,
    ) -> &'static str {
        *server.current_game_state.lock().await = game_state;
        server
            .sender
            .send(ServerMessage::UpdatedGameState)
            .await
            .unwrap();
        "OK"
    }
}
