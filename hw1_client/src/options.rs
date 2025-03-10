use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum Protocol {
    TCP,
    UDP,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum TransferAmount {
    Small,
    Big,
}

#[derive(Parser, Debug)]
pub struct Options {
    pub protocol: Protocol,
    pub message_size: u32,
    pub transfer_amount: TransferAmount,
    #[arg(short, long)]
    pub stop_and_wait: bool,
}
