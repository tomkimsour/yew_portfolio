// use serde::{Deserialize, Serialize};






pub struct LanguageCategory {
  pub categoryTitle: String,
  pub languages: Vec<Language>,
}

pub struct Language {
  pub name: String,
  pub level: String,
}

pub struct ExperienceCategory{
  pub categoryTitle:String,
  pub experiences: Vec<Experience>,
}

pub struct Experience{
  pub title: String,
  pub description: String,
  pub technos: Vec<String>,
}
