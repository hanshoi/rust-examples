use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Clone, Debug, PartialEq)]
enum LineType {
    Name,
    Command,
}

#[derive(Clone, Debug)]
struct Line {
    line_type: LineType,
    buffer: Vec<u8>,
    count: u16,
}

impl Line {
    fn new() -> Line {
        Line::from_type(LineType::Name)
    }

    fn from_type(line_type: LineType) -> Line {
        Line {
            line_type,
            buffer: Vec::new(),
            count: 0,
        }
    }

    fn is_first_character(&self) -> bool {
        self.count == 0
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn push(&mut self, byte: u8) {
        self.buffer.push(byte);
        self.count += 1;
    }

    fn to_string(self) -> String {
        String::from_utf8(self.buffer).unwrap()
    }
}

pub fn parse(content: Vec<u8>) -> HashMap<String, Command> {
    let mut commands = HashMap::new();
    let mut line = Line::new();
    let mut name = Line::new();

    for character in content {
        match character {
            // new line
            10 => {
                if line.is_empty() {
                    continue;
                }
                match line.line_type {
                    LineType::Name => {
                        name = line;
                        line = Line::from_type(LineType::Command);
                    }
                    LineType::Command => {
                        commands.insert(
                            name.clone().to_string(),
                            Command {
                                name: name.to_string(),
                                command: line.to_string(),
                            },
                        );
                        line = Line::from_type(LineType::Name);
                        name = Line::new();
                    }
                }
            }
            // Colon
            58 => {
                if line.line_type == LineType::Name {
                    continue;
                } else {
                    line.push(character.to_owned());
                }
            }
            // space or tab
            32 | 9 => {
                if line.line_type == LineType::Name {
                    continue;
                }
                if line.is_first_character() {
                    line.line_type = LineType::Command;
                } else if !line.buffer.is_empty() {
                    line.push(character.to_owned())
                }
            }
            _ => {
                line.push(character.to_owned());
            }
        }
    }

    if !line.is_empty() && line.line_type == LineType::Command {
        commands.insert(
            name.clone().to_string(),
            Command {
                name: name.to_string(),
                command: line.to_string(),
            },
        );
        line = Line::from_type(LineType::Name);
        name = Line::new();
    }
    commands
}

#[cfg(test)]
mod tests {
    use crate::file;

    #[test]
    fn test_parse() {
        let content = "command: \n\techo lol\n".as_bytes().to_vec();

        let commands = file::parse(content);
        let command = commands["command"].clone();
        assert_eq!(command.name, "command");
        assert_eq!(command.command, "echo lol");
        assert_eq!(commands.len(), 1);
    }

    #[test]
    fn test_parse_empty_lines() {
        let content = "\ncommand:\n\n \n\techo lol\n\n".as_bytes().to_vec();

        let commands = file::parse(content);
        let command = commands["command"].clone();
        assert_eq!(command.name, "command");
        assert_eq!(command.command, "echo lol");
        assert_eq!(commands.len(), 1);
    }

    #[test]
    fn test_parse_no_end_of_file_newline() {
        let content = "\ncommand:\n\n \n\techo lol".as_bytes().to_vec();

        let commands = file::parse(content);
        let command = commands["command"].clone();
        assert_eq!(command.name, "command");
        assert_eq!(command.command, "echo lol");
        assert_eq!(commands.len(), 1);
    }
}
