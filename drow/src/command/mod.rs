pub mod setup;
pub mod run;
pub mod build;
pub mod deploy;
pub mod post;
pub mod page;

pub use command::setup::setup;
pub use command::run::run;
pub use command::build::build;
pub use command::deploy::deploy;
pub use command::post::post;
pub use command::page::page;
