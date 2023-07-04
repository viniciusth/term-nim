use axum::{routing::get, Router};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use local_ip_address::local_ip;
use std::{
    io,
    time::{Duration, Instant},
};
use term_nim::App;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();

    if let Err(err) = res {
        println!("{:?}", err)
    }
    let local_ip = local_ip().unwrap();
    println!("{local_ip}");

    // build our application with a route
    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root));

    let url = format!("{}:3000", local_ip);

    // axum::Server::bind(&url.parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| app.render(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                } else if let KeyCode::Esc = key.code {
                    return Ok(());
                }
                app.handle_key(key.code);
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
