use chrono::Local;
use std::path::Path;
use uuid::Uuid;

pub struct TemplateContext {
    pub name: String,
    pub stem: String,
    pub ext: String,
    pub date: String,
    pub year: String,
    pub month: String,
    pub day: String,
    pub uuid: String,
}

impl TemplateContext {
    pub fn new(path: &Path) -> Self {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file")
            .to_string();

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file")
            .to_string();

        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let now = Local::now();
        let date = now.format("%Y%m%d").to_string();
        let year = now.format("%Y").to_string();
        let month = now.format("%m").to_string();
        let day = now.format("%d").to_string();
        let uuid = Uuid::now_v7().to_string();

        Self {
            name,
            stem,
            ext,
            date,
            year,
            month,
            day,
            uuid,
        }
    }

    pub fn render(&self, template: &str) -> String {
        template
            .replace("{name}", &self.name)
            .replace("{stem}", &self.stem)
            .replace("{ext}", &self.ext)
            .replace("{date}", &self.date)
            .replace("{year}", &self.year)
            .replace("{month}", &self.month)
            .replace("{day}", &self.day)
            .replace("{uuid}", &self.uuid)
    }
}

pub fn render_filename(template: Option<&str>, path: &Path) -> String {
    let ctx = TemplateContext::new(path);
    match template {
        Some(t) => ctx.render(t),
        None => ctx.name,
    }
}
