pub mod helloworld {
    include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));

    // Optional: re-export the useful modules
    pub use self::greeter_server::{Greeter, GreeterServer};
    pub use self::greeter_client::GreeterClient;
    pub use self::{HelloReply as OtherHelloReply, HelloRequest as OtherHelloRequest};
}
