use error_chain::error_chain;
use std::io;
use toml::de;
use toml::ser;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        TOMLSerialise(ser::Error) #[doc = "Error during serialisation"];
        TOMLDeserialise(de::Error) #[doc = "Error during deserialisation"];
    }
    errors {
    }
}
