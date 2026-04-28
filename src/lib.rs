#![warn(clippy::all, rust_2018_idioms)]


// values are hardcoded for now but I'll pull them from a db

mod structs {
    #[derive(Default)]
    pub struct Application {
        pub name: String,
        pub docker_image: String,
        pub desc: String,
        pub ip_port: String,
    }

    impl Application {
        pub fn name(&mut self) -> &mut String {
            &mut self.name
        }
        pub fn docker_image(&mut self) -> &str {
            &self.docker_image
        }
        pub fn desc(&self) -> &str {
            &self.desc
        }

    }
}

mod app;
pub use app::SelfhostingToolkit;
