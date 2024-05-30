use std::fs::File;

use inquire::{Editor, Select, Text};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Details {
    pub title: String,
    pub author: String,
    pub artist: String,
    pub description: String,
    pub genre: Vec<String>,
    pub status: Status,
}

#[derive(Debug, Default, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
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
    let detail = prompt_details();
    create_file(&detail)
}

fn prompt_details() -> Details {
    let title = Text::new("Title").prompt().unwrap_or_default();
    let author = Text::new("Author(s)").prompt().unwrap_or_default();
    let artist = Text::new("Artist(s)").prompt().unwrap_or_default();
    let description = Editor::new("Description").prompt().unwrap_or_default();
    let genre = Text::new("Genre(s)").prompt().unwrap_or_default();
    let status = Select::new("Status", Status::VARIANTS.to_vec())
        .prompt()
        .unwrap_or_default();

    Details {
        title,
        author,
        artist,
        description,
        genre: genre
            .split(',')
            .map(|s| s.trim())
            .map(|s| s.to_string())
            .collect(),
        status,
    }
}

fn create_file(details: &Details) -> std::io::Result<()> {
    let file = File::create("details.json")?;
    serde_json::to_writer_pretty(file, details)?;
    Ok(())
}
