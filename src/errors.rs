use error_chain::error_chain;
use std::io;
use toml::de;
use toml::ser;
use libloading;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        Serialise(ser::Error) #[doc = "Error during serialisation"];
        Deserialise(de::Error) #[doc = "Error during deserialisation"];
        Library(libloading::Error) #[doc = "Error with library loading"];
    }
    errors {
        InstanceExists
        NoMatchingGame
    }
}
