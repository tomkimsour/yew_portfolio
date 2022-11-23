// use serde::{Deserialize, Serialize};

pub struct FormationCategory{
  pub categoryTitle: String,
  pub formations: Formation[],
}

pub struct Formation{
  pub name: String,
  pub description: String,
  pub year: String,
  pub place: String, 
}

pub struct TechnosCategory{
  pub categoryTitle: String,
  pub technos: Techno[];
}

pub struct Techno {
  pub title: String,
  pub names: string[];
}

pub struct LanguageCategory {
  pub categoryTitle: String,
  pub languages: Language[];
}

pub struct Language {
  pub name: String,
  pub level: String,
}

pub struct ExperienceCategory{
  pub categoryTitle:String,
  pub experiences: Experience[],
}

pub struct Experience{
  pub title: String,
  pub description: String,
  pub technos: String[],
}
