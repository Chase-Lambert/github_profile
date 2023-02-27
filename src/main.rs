mod new_job;
mod profile;

use new_job::{find_new_job, MyNewJob};
use profile::{AboutMe, Project};

#[tokio::main]
async fn main() -> MyNewJob {
    println!("Hello, world!");

    let about_me = AboutMe {
        name: "Chase Lambert",

        about: "I am a self taught developer who has experience building full stack web apps. \
                I am looking for a new role using Clojure/ClojureScript or Rust. \
                I prefer remote but will consider relocation within the US. \
                I have mainly focused on web development but am open to other projects as well",

        skills: vec![
            "rust",
            "clojure(script)",
            "web development",
            "sql (mostly postgres)",
            "tailwind",
        ],

        contact: "chaselambert@gmail.com",

        projects: vec![
            Project {
                name: "lessonplanner.ai",
                // Currently deployed on free tier so first load may be slow
                url: "https://lessonplanner-rust.onrender.com/",
                repo: "https://github.com/chase-lambert/lesson_planner",
                desc: "Full stack Rust app \
                       Backend uses Axum with a Postgres db \
                       Frontend uses Leptos with Tailwind CSS for styling. \
                       Uses the OpenAI api to help teachers build lesson plans.",
            },
            Project {
                name: "distance finder",
                // Currently deployed on free tier so first load may be slow
                url: "https://distancefinder.onrender.com/",
                repo: "https://github.com/chase-lambert/distancefinder",
                desc: "Final project for Harvard's CS50 course \
                       Built using Python with Flask",
            },
        ],
    };

    let new_job = find_new_job(about_me).await?;

    Ok(new_job)
}
