use std::{
    fs::File,
    io::{BufWriter, Write},
};

use inquire::{Confirm, Editor, MultiSelect, Select, Text};

use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Debug, Clone, Default, Serialize)]
struct Details<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub artist: &'a str,
    pub description: &'a str,
    pub genre: Vec<Genre>,
    pub status: Status,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
enum Genre {
    #[default]
    Unknown,
    Action,
    Adventure,
    Comedy,
    Drama,
    Ecchi,
    Fantasy,
    Hentai,
    Horror,
    MahouShoujo,
    Mecha,
    Music,
    Mystery,
    Psychological,
    Romance,
    SciFi,
    SlifeOfLife,
    Sports,
    Supernatural,
    Thriller,
}

impl Genre {
    const VARIANTS: &'static [Self] = &[
        Self::Unknown,
        Self::Action,
        Self::Adventure,
        Self::Comedy,
        Self::Drama,
        Self::Ecchi,
        Self::Fantasy,
        Self::Hentai,
        Self::Horror,
        Self::MahouShoujo,
        Self::Mecha,
        Self::Music,
        Self::Mystery,
        Self::Psychological,
        Self::Romance,
        Self::SciFi,
        Self::SlifeOfLife,
        Self::Sports,
        Self::Supernatural,
        Self::Thriller,
    ];
}

impl std::fmt::Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MahouShoujo => "Mahou Shoujo".to_string(),
                Self::SciFi => "Sci-Fi".to_string(),
                Self::SlifeOfLife => "Slice of Life".to_string(),
                other => format!("{other:?}"),
            }
        )
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize_repr)]
#[repr(u8)]
enum Status {
    #[default]
    Unknown = 0,
    Ongoing = 1,
    Completed = 2,
    Cancelled = 5,
    Hiatus = 6,
}

impl Status {
    const VARIANTS: &'static [Self] = &[
        Self::Unknown,
        Self::Ongoing,
        Self::Completed,
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
    let title = Text::new("Title")
        .with_placeholder("Frieren: Beyond Journey's End")
        .prompt()
        .unwrap_or_default();

    let author = Text::new("Author(s)")
        .with_placeholder("Kanehito Yamada")
        .with_help_message("if there are multiple, each author should be separated by a comma")
        .prompt()
        .unwrap_or_default();

    let artist = Text::new("Artist(s)")
        .with_placeholder("Tsukasa Abe")
        .with_help_message("if there are multiple, each artist should be separated by a comma")
        .prompt()
        .unwrap_or_default();

    let description = Editor::new("Description").prompt().unwrap_or_default();

    let genre = MultiSelect::new("Genre", Genre::VARIANTS.to_vec())
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
        genre,
        status,
    };

    let mut stdout = std::io::stdout();

    if let Ok(true) = Confirm::new("Write to file? [./details.json]")
        .with_default(false)
        .prompt()
    {
        let file = File::create("details.json")?;
        let bufw = BufWriter::new(file);
        serde_json::to_writer_pretty(bufw, &detail)?;
        writeln!(stdout, "Written to ./details.json")?;
        return Ok(());
    }

    writeln!(stdout, "{}", serde_json::to_string_pretty(&detail)?)?;

    Ok(())
}
