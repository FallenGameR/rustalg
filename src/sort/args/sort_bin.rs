use anyhow::{Result, anyhow};
use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Algorithm {
    Selection,
    Insertion,
}

