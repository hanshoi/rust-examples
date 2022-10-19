use crate::io;
use anyhow::anyhow;

pub type History = Vec<String>;

trait FromBytes {
    fn from_bytes(bytes: Vec<u8>) -> History;
}

impl FromBytes for History {
    fn from_bytes(bytes: Vec<u8>) -> History {
        bytes
            .split(|byte| *byte == 10) // split on newline
            .map(|line| String::from_utf8(line.to_vec()).unwrap())
            .collect()
    }
}

pub fn get_history() -> History {
    let shell = Shell::from_str(setenv::get_shell().get_name()).unwrap();
    match shell {
        Shell::Bash => get_bash_history(),
        Shell::Zsh => get_zsh_history(),
    }
}

pub fn get_last_command() -> String {
    get_history()
        .into_iter()
        .filter(|x| !x.is_empty())
        .last()
        .unwrap()
}

fn get_bash_history() -> History {
    History::from_bytes(io::read_home_dir(".bash_history").unwrap())
}

fn get_zsh_history() -> History {
    zsh::process_history(io::read_home_dir(".zsh_history").unwrap())
}

#[derive(Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
}

impl Shell {
    pub fn from_str(string: &str) -> anyhow::Result<Self> {
        match string {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            _ => Err(anyhow!("Not implemented for {}", string)),
        }
    }
}

mod zsh {
    use super::{FromBytes, History};
    use regex::Regex;

    pub fn process_history(history: Vec<u8>) -> History {
        remove_timestamps(History::from_bytes(unmetafy(history)))
    }

    fn unmetafy(mut bytestring: Vec<u8>) -> Vec<u8> {
        /* Unmetafying zsh history requires looping over the bytestring, removing
         * each encountered Meta character, and XOR-ing the following byte with 32.
         *
         * For instance:
         *
         * Input: ('a', 'b', 'c', Meta, 'd', 'e', 'f')
         * Wanted: ('a', 'b', 'c', 'd' ^ 32, 'e', 'f')
         */
        const ZSH_META: u8 = 0x83;

        for index in (0..bytestring.len()).rev() {
            if bytestring[index] == ZSH_META {
                bytestring.remove(index);
                bytestring[index] ^= 32;
            }
        }
        bytestring
    }

    fn remove_timestamps(history: History) -> History {
        /* The preceding metadata needs to be stripped
         * because zsh history entries look like below:
         *
         * `: 1330648651:0;sudo reboot`
         */
        let r = Regex::new(r"^: \d{10}:\d;").unwrap();
        history
            .iter()
            .map(|line| r.replace(line, "").into_owned())
            .collect()
    }
}
