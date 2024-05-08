use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub struct Magic {
    pub offsets: Vec<Offset>,
}

pub struct Offset {
    pub at: String,
    pub first: Vec<FirstByte>,
}

pub struct FirstByte {
    pub byte: String,
    pub matches: Vec<Match>,
}

pub struct Match {
    pub bytes: String,
    pub ext: String,
    pub cat: String,
    pub os: String,
    pub info: String,
}

pub fn parse(file: &str) -> Magic {
    let file = File::open(file).unwrap();
    let file = BufReader::new(file);

    let mut magic = Magic {
        offsets: Vec::new(),
    };

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let attrs: HashMap<_, _> = attributes
                    .iter()
                    .map(|attr| (attr.name.local_name.clone(), attr.value.clone()))
                    .collect();
                match name.local_name.as_str() {
                    "offset" => {
                        let at = attrs.get("at").unwrap().clone();
                        let offset = Offset {
                            at,
                            first: Vec::new(),
                        };
                        magic.offsets.push(offset);
                    }
                    "first" => {
                        let byte = attrs.get("byte").unwrap().clone();
                        let first = FirstByte {
                            byte,
                            matches: Vec::new(),
                        };
                        if let Some(offset) = magic.offsets.last_mut() {
                            offset.first.push(first);
                        }
                    }
                    "match" => {
                        let bytes = attrs.get("bytes").unwrap_or(&"-".to_string()).clone();
                        let info = attrs.get("info").unwrap_or(&"-".to_string()).clone();
                        let cat = attrs.get("cat").unwrap_or(&"-".to_string()).clone();
                        let os = attrs.get("os").unwrap_or(&"-".to_string()).clone();
                        let ext = attrs.get("ext").unwrap_or(&"-".to_string()).clone();
                        let m = Match {
                            bytes,
                            ext,
                            cat,
                            os,
                            info,
                        };
                        if let Some(offset) = magic.offsets.last_mut() {
                            if let Some(first) = offset.first.last_mut() {
                                first.matches.push(m);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    magic
}
