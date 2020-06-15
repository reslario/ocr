use std::{
    fmt,
    path::Path
};

pub struct ImageFile {
    pub base64: String,
    pub filetype: Filetype
}

#[derive(serde::Serialize)]
pub struct Filetype(String);

impl Default for Filetype {
    fn default() -> Self {
        Filetype("png".into())
    }
}

impl fmt::Display for Filetype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for Filetype {
    fn from(s: &str) -> Self {
        let s = match s {
            "jpeg" => "jpg",
            _ => s
        }.into();

        Filetype(s)
    }
}

pub fn load_image(path: impl AsRef<Path>) -> std::io::Result<ImageFile> {
    let filetype = filetype(path.as_ref());
    Ok(ImageFile {
        base64: image(path.as_ref(), &filetype)?,
        filetype
    })
}

fn image(path: &Path, filetype: &Filetype) -> std::io::Result<String> {
    std::fs::read(path)
        .map(|bytes| format!(
            "data:image/{};base64,{}",
            filetype,
            base64::encode(bytes)
        ))
}

fn filetype(path: &Path) -> Filetype {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(Filetype::from)
        .unwrap_or_default()
}