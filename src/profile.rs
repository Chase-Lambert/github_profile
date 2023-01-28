#[derive(Debug)]
pub struct AboutMe<'a> {
    pub name: &'a str,
    pub about: &'a str,
    pub skills: Vec<&'a str>,
    pub contact: &'a str,
    pub projects: Vec<Project<'a>>,
}

#[derive(Debug)]
pub struct Project<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub repo: &'a str,
    pub desc: &'a str,
}
