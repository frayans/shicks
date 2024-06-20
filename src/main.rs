use std::fs::File;

use inquire::{Confirm, Editor, Select, Text};

use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Details<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub artist: &'a str,
    pub description: &'a str,
    pub genre: Vec<&'a str>,
    pub status: Status,
}

#[derive(Debug, Default, Clone, Copy, Serialize_repr)]
#[repr(u8)]
pub enum Status {
    #[default]
    Unknown = 0,
    Ongoing = 1,
    Completed = 2,
    Licensed = 3,
    Finished = 4,
    Cancelled = 5,
    Hiatus = 6,
}

impl Status {
    pub const VARIANTS: &'static [Self] = &[
        Self::Unknown,
        Self::Ongoing,
        Self::Completed,
        Self::Licensed,
        Self::Cancelled,
        Self::Hiatus,
    ];
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

fn main() -> std::io::Result<()> {
    let title = Text::new("Title").prompt().unwrap_or_default();
    let author = Text::new("Author(s)").prompt().unwrap_or_default();
    let artist = Text::new("Artist(s)").prompt().unwrap_or_default();
    let description = Editor::new("Description").prompt().unwrap_or_default();
    let genre = Text::new("Genre(s)")
        .with_help_message("Genres must be separated with a comma (genre1, genre2, etc)")
        .prompt()
        .unwrap_or_default();
    let status = Select::new("Status", Status::VARIANTS.to_vec())
        .prompt()
        .unwrap_or_default();

    let detail = Details {
        title: &title,
        author: &author,
        artist: &artist,
        description: &description,
        genre: genre.split(',').map(|s| s.trim()).collect(),
        status,
    };

    if let Ok(true) = Confirm::new("Write to file? [./details.json]")
        .with_default(false)
        .prompt()
    {
        let file = File::create("details.json")?;
        serde_json::to_writer_pretty(file, &detail)?;
        println!("Written to ./details.json")
    }

    println!("{}", serde_json::to_string_pretty(&detail)?);

    Ok(())
}
