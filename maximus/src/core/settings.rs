/*
   Appellation: settings
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use components::*;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
}

impl Configuration {
    pub fn new() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder();

        builder.try_deserialize()?
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Configuration(application={}, database={}, logger={})",
            self.application, self.database, self.logger
        )
    }
}

mod components {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Application {
        pub mode: String,
        pub name: String,
    }

    impl std::fmt::Display for Application {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Application(mode={}, name={})", self.mode, self.name)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Database {
        pub name: String,
        pub uri: String,
    }

    impl std::fmt::Display for Database {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Database(name={}, uri={})", self.name, self.uri)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Logger {
        pub level: String,
    }

    impl std::fmt::Display for Logger {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Logger(level={})", self.level)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
