use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::project::project::{Project, ProjectProps};

#[derive(Clone, PartialEq)]
struct ProjectsStruct {
    projects: Vec<ProjectProps>,
}

#[styled_component(Projects)]
pub fn projects() -> Html {
    let projects = ProjectsStruct{
        projects: vec![
            ProjectProps{
                key:1,
                image: "assets/robocup_2022.JPG".to_string(),
                title: "Robocup@Home 🤖💙".to_string(),
                description: "The Robocup@Home is a league within the famous international Robocup competition. The goal of this league is to drive research on robot assistant applications. During the competition, the robot will have to execute various tasks such as welcoming guests and introducing them to everyone.
                While working on this very broad project, I had to manage the team github and code integration on the robot as well as the OS of the robot. I also worked on a navigation module using various libraries, worked on semantic parsing for an NLP task and integrate our project in a simulation environment.".to_string(),
                tech_list: vec![
                    "ROS".to_string(),
                    "NaoQi".to_string(),
                    "ORB-SLAM".to_string(),
                    "TEB-local-planner".to_string(),
                    "Octomap".to_string(),
                    "Gentoo".to_string(),
                    "Unreal".to_string(),
                    "Spacy".to_string(),
                    "Vosk".to_string(),
                    "CMake".to_string(),
                    "C++".to_string(),
                    "Python".to_string(),
                ],
                year:"2021 - Now".to_string(),
                url: "https://www.enib.fr/~robobreizh/src/en/project_home_en.html".to_string(),
            },
            ProjectProps{
                key:2,
                image: "assets/othello.png".to_string(),
                title: "Othello".to_string(),
                description: "School project where the goal was to create an othello ai using min-max with alpha pruning algorithm".to_string(),
                tech_list: vec![
                    "Go".to_string(),
                    "Java".to_string()
                ],
                year:"2021".to_string(),
                url: "https://github.com/tomkimsour/Othello".to_string(),
            },
            ProjectProps{
                key:3,
                image: "assets/video_converter.png".to_string(),
                title: "Cloud video converter".to_string(),
                description: "This project was an introduction to cloud services. During the course we had to present a small state of the art of a field of cloud computing. This project was related to the course and we had to implement a video format converter that would scale up and down depending on the number of current requests.".to_string(),
                tech_list: vec![
                    "React".to_string(),
                    "Tailwind".to_string(),
                    "Flask".to_string(),
                    "Kubernetes".to_string(),
                    "Graphana".to_string(),
                    "Docker".to_string(),
                    "GCP".to_string(),
                ],
                year:"2021".to_string(),
                url: "https://github.com/tomkimsour/video-converter".to_string(),
            },
            ProjectProps{
                key:4,
                image: "assets/advent_of_code.jpeg".to_string(),
                title: "Advent of code".to_string(),
                description: "The advent of code is a fun and challenging yearly event happening during december. Where you would be given 2 problems to solve every day until that beloved christmas day.".to_string(),
                tech_list: vec![
                    "Go".to_string(),
                    "Python".to_string(),
                    "Rust".to_string(),
                ],
                year:"2020 - 2022".to_string(),
                url: "https://github.com/tomkimsour/video-converter".to_string(),
            },
            ProjectProps{
                key:5,
                image: "assets/multiarm_bandit.png".to_string(),
                title: "Qlearning and multi arm bandit".to_string(),
                description: "This project was an introduction to reinforcement learning using Q learning and multi arm bandit.".to_string(),
                tech_list: vec![
                    "Python".to_string(),
                    "numpy".to_string(),
                    "matplotlib".to_string(),
                ],
                year:"2021".to_string(),
                url: "https://github.com/tomkimsour/Reinforcement_Learning".to_string(),
            },
            ProjectProps{
                key:6,
                image: "assets/tz_converter.png".to_string(),
                title: "Time zone converter CLI".to_string(),
                description: "This is a small command line interface writtent in rust that would convert time from one timezone to another".to_string(),
                tech_list: vec![
                    "Rust".to_string(),
                ],
                year:"2022".to_string(),
                url: "https://github.com/tomkimsour/tz-range-converter".to_string(),
            }
    ]};

    html! {
        <>
            <div id="project" class="flex flex-col flex-nowrap box-border px-110px min-h-screen  min-h-full">
                <h1>{"Selected projects"}</h1>
                <div class="grid grid-cols-3 gap-y-10 gap-x-6">
                {
                    projects.projects.iter().map(|project|{
                        html!(
                            <Project ..project.clone() />
                        )}).collect::<Html>()
                }
                </div>
            </div>
        </>
    }
}
