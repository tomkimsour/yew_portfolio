use super::{formation::{Formation,FormationProps, FormationStruct}, technos::{Technos,TechnosProps, TechnosStruct}, languages::{LanguageProps, LanguageStruct, Languages}, experience::{Experience,ExperienceProps, ExperienceStruct}}; 
use crate::components::button::button::DownloadButton;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    // Instantioate data
    let formation_props = FormationProps {
        category_title: "Education".to_string(),
        formations: vec![
            FormationStruct {
                id: 1,
                name: "Master 2 in computer science".to_string(),
                description: "Artificial Intelligence and algorithm optimization".to_string(),
                year: "2021-2022".to_string(),
                place: "Umeå Universitet, Sweden".to_string(),
            },
            FormationStruct {
                id: 2,
                name: "Master degree in computer science".to_string(),
                description: "Autonomous, Interactive and Intelligent Systems".to_string(),
                year: "2020-2022".to_string(),
                place: "Université de Bretagne occidentale, France".to_string(),
            },
            // FormationStruct {
            //     id: 3,
            //     name: "Bachelor degree in computer science".to_string(),
            //     description: "Fondations and applications".to_string(),
            //     year: "2017-2020".to_string(),
            //     place: "Université de Bretagne occidentale, France".to_string(),
            // },
        ],
    };

    let technos_props = TechnosProps{
        category_title: "Technos".to_string(),
        technos: vec![
            TechnosStruct{
                key: 1,
                title:"Languages".to_string(),
                names: vec![
                        "c".to_string(),
                        "c++".to_string(),
                        "java".to_string(),
                        "python".to_string(),
                        "typescript".to_string(),
                        "javascript".to_string(),
                        "go".to_string(),
                        "Rust".to_string(),
                        "php".to_string()
                    ]
            },
            TechnosStruct{
                key: 2,
                title:"Tools & Frameworks".to_string(),
                names: vec![
                    "docker".to_string(),
                    "kubernetes".to_string(), 
                    "ROS".to_string(), 
                    "keras".to_string(), 
                    "tensorflow".to_string()
                    ]
            },
            TechnosStruct{
                key:3,
                title:"Database".to_string(),
                names: vec![
                    "mysql".to_string(),
                    "postgresql".to_string(),
                    "sqlite".to_string()
                    ]
            }
        ],
    };

    let language_props = LanguageProps{
        category_title: "Languages".to_string(),
        languages : vec![LanguageStruct{
            key:1,
            name:"French".to_string(),
            level:"Native".to_string()
        },
        LanguageStruct{
            key:2,
            name:"English".to_string(),
            level:"Fluent".to_string()
        },
        LanguageStruct{
            key:3,
            name:"German".to_string(),
            level:"Elementary".to_string()
        },
        LanguageStruct{
            key:4,
            name:"Mandarin".to_string(),
            level:"Elementary".to_string()
        }
        ]
    };

    let experience_props = ExperienceProps{
        category_title : "Experiences and projects".to_string(),
        experiences : vec![
            ExperienceStruct{
                key : 1,
                title:"Internship at CERV".to_string(),
                description:"research and implementation of navigation module and NLP for the competition RoboCup@home".to_string(),
                technos: vec![
                    "ROS".to_string(),
                    "C++".to_string(),
                    "Python".to_string()
                ],
            },
            ExperienceStruct{
                key : 2,
                title:"Internship at Ceva Ecat-id".to_string(),
                description:"web application development".to_string(),
                technos: vec![
                    "Express.js".to_string(),
                    "SQLite".to_string()
                ],
            },
            ExperienceStruct{
                key : 3,
                title:"Video converter API".to_string(),
                description:"".to_string(),
                technos: vec![
                    "GCP".to_string(), 
                    "Kubernetes".to_string(), 
                    "Next.js".to_string(), 
                    "Flask".to_string(), 
                    "Prometheus".to_string()
                ],
            },
            ExperienceStruct{
                key : 4,
                title:"Othello min max AI".to_string(),
                description:"".to_string(),
                technos: vec![
                    "Go".to_string()
                ],
            }
            
        ]
    };

    html! {
        <div class="h-full min-h-screen min-h-full grid gap-2 content-center" id="about">
            <h1>{"Research engineer"}</h1>
            <div class="grid grid-rows-2 grid-cols-4 gap-4 grid-flow-row-dense">
                <div class="col-span-2 row-span-1">
                    <Formation
                        category_title={formation_props.category_title}
                        formations={formation_props.formations}
                    />
                </div>
                <div class="col-span-2 row-span-2">
                    <Experience
                        category_title={experience_props.category_title}
                        experiences={experience_props.experiences}
                    />
                </div>
                <div class="row-span-1">
                    <Technos
                        category_title={technos_props.category_title}
                        technos={technos_props.technos}
                    />
                </div>
                <div class="row-span-1">
                    <Languages
                        category_title={language_props.category_title}
                        languages={language_props.languages}
                    />
                </div>
            </div>
            <div class="flex flex-col justify-item-start h-1/6 py-8">
                <DownloadButton name="DOWNLOAD RESUME" link="assets/resume_thomas_ung.pdf"/>
            </div>
        </div>
    }
}