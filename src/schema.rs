use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub date_range: String,
    pub location: String
}

#[derive(Deserialize, Debug)]
pub struct Experience {
    pub title: String,
    pub place: String,
    pub location: String,
    pub date_range: String,
    pub bullet_points: Vec<BulletPoint>
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub technologies: Vec<Technology>,
    pub bullet_points: Vec<BulletPoint>,
    pub date_range: String
}

#[derive(Deserialize, Debug)]
pub struct Technology {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct BulletPoint {
    pub description: String
}

#[derive(Deserialize, Debug)]
pub struct Langugaes {
    pub languages: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Frameworks { 
    pub frameworks: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Tools {
    pub tools: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Libraries {
    pub libraries: Vec<String>
}
