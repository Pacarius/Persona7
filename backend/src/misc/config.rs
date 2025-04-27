use clap::Parser;

// Define constants as defaults
const DEFAULT_TEXT_MODEL: &str = "llama3.2";
const DEFAULT_EMBEDDING_MODEL: &str = "nomic-embed-text";
const DEFAULT_OLLAMA_IP: &str = "192.168.50.84";
const DEFAULT_OLLAMA_PORT: &str = "11434";
const DEFAULT_TIME_STEP: i64 = 60;
const DEFAULT_TICK_COOLDOWN_MS: u64 = 1000;
const DEFAULT_SQLITE_PATH: &str = "runs";
const DEFAULT_OLLAMA_SEED: i64 = 5;

// Config structure with command line parsing
#[derive(Parser, Debug)]
#[command(author, version, about = "Persona 6 Backend Server")]
pub struct Config {
    /// Text model to use
    #[arg(long, default_value = DEFAULT_TEXT_MODEL, env)]
    pub text_model: String,

    /// Embedding model to use
    #[arg(long, default_value = DEFAULT_EMBEDDING_MODEL, env)]
    pub embedding_model: String,

    /// Ollama server IP address
    #[arg(long, default_value = DEFAULT_OLLAMA_IP, env)]
    pub ollama_ip: String,

    /// Ollama server port
    #[arg(long, default_value = DEFAULT_OLLAMA_PORT, env)]
    pub ollama_port: String,

    /// Time step in seconds
    #[arg(long, default_value_t = DEFAULT_TIME_STEP, env)]
    pub time_step: i64,

    /// Tick cooldown in milliseconds
    #[arg(long, default_value_t = DEFAULT_TICK_COOLDOWN_MS, env)]
    pub tick_cooldown_ms: u64,

    /// Sqlite file path
    #[arg(long, default_value = DEFAULT_SQLITE_PATH, env)]
    pub sqlite_path: String,

    ///Ollama Seed
    #[arg(long, default_value_t = DEFAULT_OLLAMA_SEED, env)]
    pub ollama_seed: i64,
}