use clap::Parser;

// Define constants as defaults
const DEFAULT_TEXT_MODEL: &str = "llama3.2";
const DEFAULT_EMBEDDING_MODEL: &str = "nomic-embed-text";
const DEFAULT_OLLAMA_IP: &str = "192.168.50.84";
const DEFAULT_OLLAMA_PORT: &str = "11434";
const DEFAULT_TIME_STEP: i64 = 60;
const DEFAULT_TICK_COOLDOWN_MS: u64 = 1000;
const SQLITE_PATH: &str = "runs";

// Config structure with command line parsing
#[derive(Parser, Debug)]
#[command(author, version, about = "Persona 6 Backend Server")]
pub struct Config {
    /// Text model to use
    #[arg(long, default_value = DEFAULT_TEXT_MODEL)]
    pub text_model: String,

    /// Embedding model to use
    #[arg(long, default_value = DEFAULT_EMBEDDING_MODEL)]
    pub embedding_model: String,

    /// Ollama server IP address
    #[arg(long, default_value = DEFAULT_OLLAMA_IP)]
    pub ollama_ip: String,

    /// Ollama server port
    #[arg(long, default_value = DEFAULT_OLLAMA_PORT)]
    pub ollama_port: String,

    /// Time step in seconds
    #[arg(long, default_value_t = DEFAULT_TIME_STEP)]
    pub time_step: i64,

    /// Tick cooldown in milliseconds
    #[arg(long, default_value_t = DEFAULT_TICK_COOLDOWN_MS)]
    pub tick_cooldown_ms: u64,

    /// Sqlite file path
    #[arg(long, default_value = SQLITE_PATH)]
    pub sqlite_path: String,
}