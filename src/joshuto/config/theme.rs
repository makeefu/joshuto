extern crate toml;
extern crate xdg;

use std::collections::HashMap;
use std::fs;
use std::process;

#[derive(Debug, Deserialize, Clone)]
pub struct JoshutoColorPair {
    pub id: i16,
    pub fg: i16,
    pub bg: i16,
}

impl JoshutoColorPair {
    pub fn new(id: i16, fg: i16, bg: i16) -> Self
    {
        JoshutoColorPair {
            id,
            fg,
            bg,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JoshutoColorTheme {
    pub colorpair: i16,
    pub bold: bool,
    pub underline: bool,
    pub prefix: Option<String>,
    pub prefixsize: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawTheme {
    colorpair: Option<Vec<JoshutoColorPair>>,
    selection: Option<JoshutoColorTheme>,
    executable: Option<JoshutoColorTheme>,
    regular: Option<JoshutoColorTheme>,
    directory: Option<JoshutoColorTheme>,
    link: Option<JoshutoColorTheme>,
    socket: Option<JoshutoColorTheme>,
    ext: Option<HashMap<String, JoshutoColorTheme>>,
}

impl JoshutoRawTheme {
    pub fn flatten(self) -> JoshutoTheme
    {
        let colorpair = match self.colorpair {
                Some(s) => s,
                None => {
                    let mut colorpair: Vec<JoshutoColorPair> = Vec::with_capacity(10);
                    colorpair.push(JoshutoColorPair::new(2, 2, -1));
                    colorpair.push(JoshutoColorPair::new(3, 3, -1));
                    colorpair.push(JoshutoColorPair::new(4, 4, -1));
                    colorpair.push(JoshutoColorPair::new(5, 5, -1));
                    colorpair.push(JoshutoColorPair::new(6, 6, -1));
                    colorpair
                }
            };

        let selection = self.selection.unwrap_or(JoshutoColorTheme {
            colorpair: 3,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            });

        let executable = self.executable.unwrap_or(JoshutoColorTheme {
            colorpair: 2,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            });

        let regular = self.regular.unwrap_or(JoshutoColorTheme {
            colorpair: 0,
            bold: false,
            underline: false,
            prefix: None,
            prefixsize: None,
            });

        let directory = self.directory.unwrap_or(JoshutoColorTheme {
            colorpair: 4,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            });

        let link = self.link.unwrap_or(JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            });

        let socket = self.socket.unwrap_or(JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            });

        let ext = self.ext.unwrap_or(HashMap::new());

        JoshutoTheme {
            colorpair,
            regular,
            directory,
            selection,
            executable,
            link,
            socket,
            ext,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JoshutoTheme {
    pub colorpair: Vec<JoshutoColorPair>,
    pub regular: JoshutoColorTheme,
    pub selection: JoshutoColorTheme,
    pub directory: JoshutoColorTheme,
    pub executable: JoshutoColorTheme,
    pub link: JoshutoColorTheme,
    pub socket: JoshutoColorTheme,
    pub ext: HashMap<String, JoshutoColorTheme>
}

impl JoshutoTheme {
    pub fn new() -> Self
    {
        let mut colorpair: Vec<JoshutoColorPair> = Vec::with_capacity(10);
        colorpair.push(JoshutoColorPair::new(2, 2, -1));
        colorpair.push(JoshutoColorPair::new(3, 3, -1));
        colorpair.push(JoshutoColorPair::new(4, 4, -1));
        colorpair.push(JoshutoColorPair::new(5, 5, -1));
        colorpair.push(JoshutoColorPair::new(6, 6, -1));

        let selection = JoshutoColorTheme {
            colorpair: 3,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let executable = JoshutoColorTheme {
            colorpair: 2,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let regular = JoshutoColorTheme {
            colorpair: 0,
            bold: false,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let directory = JoshutoColorTheme {
            colorpair: 4,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let link = JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        let socket = JoshutoColorTheme {
            colorpair: 6,
            bold: true,
            underline: false,
            prefix: None,
            prefixsize: None,
            };

        JoshutoTheme {
            colorpair,
            selection,
            executable,
            regular,
            directory,
            link,
            socket,
            ext: HashMap::new(),
        }

    }

    fn read_config() -> Option<JoshutoRawTheme>
    {
        match xdg::BaseDirectories::with_profile(::PROGRAM_NAME, "") {
            Ok(dirs) => {
                let config_path = dirs.find_config_file(::THEME_FILE)?;
                match fs::read_to_string(&config_path) {
                    Ok(config_contents) => {
                        match toml::from_str(&config_contents) {
                            Ok(config) => {
                                Some(config)
                            },
                            Err(e) => {
                                eprintln!("Error parsing theme file: {}", e);
                                process::exit(1);
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    },
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }

    pub fn get_config() -> Self
    {
        match Self::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                Self::new()
            }
        }
    }
}
