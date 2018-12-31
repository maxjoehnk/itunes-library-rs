extern crate xml;

use std::path::Path;
use property_list::{PropertyListValue, PropertyListDict};
use error::Error;

mod error;
mod property_list;

#[derive(Debug, Default)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
}

#[derive(Debug, Default)]
pub struct ItunesLibrary {
    pub version: Version,
    pub application_version: Option<String>,
    pub date: Option<String>,
    pub music_folder: Option<String>,
    pub tracks: Vec<ItunesTrack>,
    pub playlists: Vec<ItunesPlaylist>,
}

impl ItunesLibrary {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<ItunesLibrary, Error> {
        let dict = property_list::read_property_list(path)?;

        let library = ItunesLibrary {
            version: Version {
                minor: dict.get("Minor Version")
                    .and_then(property_list::to_i32)
                    .unwrap(),
                major: dict.get("Major Version")
                    .and_then(property_list::to_i32)
                    .unwrap(),
            },
            application_version: dict.get("Application Version")
                .and_then(property_list::to_string),
            date: dict.get("Date")
                .and_then(property_list::to_string),
            music_folder: dict.get("Music Folder")
                .and_then(property_list::to_string),
            tracks: dict.get("Tracks")
                .map(|tracks| match tracks {
                    PropertyListValue::Dict(dict) => {
                        dict.values()
                            .map(|track_value| match track_value {
                                PropertyListValue::Dict(track_dict) => ItunesTrack::from_dict(track_dict),
                                _ => unreachable!()
                            })
                            .collect()
                    },
                    _ => Vec::new()
                })
                .unwrap_or_default(),
            ..ItunesLibrary::default()
        };

        Ok(library)
    }
}
#[derive(Debug, Default)]
pub struct ItunesTrack {
    pub id: i32,
    pub name: String,
    pub artist: Option<String>,
    pub album: Option<String>
}

impl ItunesTrack {
    fn from_dict(dict: &PropertyListDict) -> ItunesTrack {
        ItunesTrack {
            id: dict.get("Track ID")
                .and_then(property_list::to_i32)
                .unwrap(),
            name: dict.get("Name")
                .and_then(property_list::to_string)
                .unwrap(),
            album: dict.get("Album")
                .and_then(property_list::to_string),
            artist: dict.get("Artist")
                .and_then(property_list::to_string)
        }
    }
}

#[derive(Debug)]
pub struct ItunesPlaylist {
    pub id: i32
}
