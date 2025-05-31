use thiserror::Error;
use octocrab::Error as OctocrabError;
use std::env::VarError;
use std::io;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Erreur GitHub : {0}")]
    GitHub(#[from] OctocrabError),

    #[error("Erreur d'environnement : {0}")]
    EnvVar(#[from] VarError),

    #[error("Erreur I/O : {0}")]
    Io(#[from] io::Error),

    #[error("Erreur personnalisée : {0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, BotError>;
