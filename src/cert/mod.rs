// SPDX-License-Identifier: Apache-2.0

pub(crate) mod export;
pub(crate) mod fetch;
pub(crate) mod import;
pub(crate) mod verify;

use std::str::FromStr;

use structopt::StructOpt;

use anyhow::{anyhow, Error};

#[derive(StructOpt)]
pub enum EncodingFormat {
    #[structopt(about = "Certificates are encoded in DER format")]
    Der,

    #[structopt(about = "Certificates are encoded in PEM format")]
    Pem,
}

impl std::fmt::Display for EncodingFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Der => "der",
                Self::Pem => "pem",
            }
        )
    }
}

impl FromStr for EncodingFormat {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "der" => Ok(Self::Der),
            "pem" => Ok(Self::Pem),
            _ => Err(anyhow!("unrecognized certificate encoding format")),
        }
    }
}
