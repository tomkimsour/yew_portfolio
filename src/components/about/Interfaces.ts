
export interface FormationCategory{
  categoryTitle: string;
  formations: Formation[];
}

export interface Formation{
  name: string;
  description: string;
  year: string;
  place: string; 
}

export interface TechnosCategory{
  categoryTitle: string;
  technos: Techno[];
}

export interface Techno {
  title: string;
  names: string[];
}

export interface LanguageCategory {
  categoryTitle: string;
  languages: Language[];
}

export interface Language {
  name: string;
  level: string;
}

export interface ExperienceCategory{
  categoryTitle:string;
  experiences: Experience[];
}

export interface Experience{
  title: string;
  description: string;
  technos: string[];
}