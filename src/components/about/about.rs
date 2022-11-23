use yew::prelude::*;
use crate::components::about::formation;

#[function_component(About)]
pub fn about() -> Html {
    html!{
        <div class="h-full about" id="about">
          <h1>{"Research Engineer"}</h1>
          <Formation
            categoryTitle={data.formationCategory.categoryTitle}
            formations={data.formationCategory.formations}
          />
          <Technos
            categoryTitle={data.technoCategory.categoryTitle}
            technos={data.technoCategory.technos}
          />
          <Languages
            categoryTitle={data.languageCategory.categoryTitle}
            languages={data.languageCategory.languages}
          />

          <Experiences
            categoryTitle={data.experienceCategory.categoryTitle}
            experiences={data.experienceCategory.experiences}
          />
        </div>
    }
}

// import data from "../../assets/about.json" assert { type: "json" };
// import Technos from "./Technos.tsx";
// import Formation from "./Formation.tsx";
// import Languages from "./Languages.tsx";
// import Experiences from "./Experiences.tsx";

// const About = () => {
//   return (
//     <div className="h-full about" id="about">
//       <h1>Research Engineer</h1>
//       <Formation
//         categoryTitle={data.formationCategory.categoryTitle}
//         formations={data.formationCategory.formations}
//       />
//       <Technos
//         categoryTitle={data.technoCategory.categoryTitle}
//         technos={data.technoCategory.technos}
//       />
//       <Languages
//         categoryTitle={data.languageCategory.categoryTitle}
//         languages={data.languageCategory.languages}
//       />

//       <Experiences
//         categoryTitle={data.experienceCategory.categoryTitle}
//         experiences={data.experienceCategory.experiences}
//       />
//     </div>
//   );
// };

