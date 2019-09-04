use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    name: String,
    email: String,
    phone: String,
    website: String,
    availability: String,
    interests: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Addresses {
    local: String,
    permanent: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EducationInfo {
    name: String,
    time: String,
    school: String,
    candidate: String,
    graduate: Option<String>,
    honors: Option<Vec<String>>,
    activities: Option<Vec<String>>,
    courses: Option<Vec<String>>
}

type Education = Vec<EducationInfo>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperienceInfo {
    name: String,
    location: String,
    title: String,
    time: String,
    bullets: Vec<String>
}

type Experience = Vec<ExperienceInfo>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerKnowledge {
    languages: Vec<String>,
    databases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalProjectInfo {
    name: String,
    status: String,
    bullets: Vec<String>
}

type PersonalProjects = Vec<PersonalProjectInfo>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    info: Info,
    education: Education,
    experience: Experience,
    #[serde(alias = "Computer Knowledge")]
    computer_knowledge: ComputerKnowledge,
    #[serde(alias = "Personal Projects")]
    personal_projects: PersonalProjects
}