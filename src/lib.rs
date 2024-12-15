pub(crate) mod commands;
pub(crate) mod constants;
pub(crate) mod detect;
pub(crate) mod tests;

// re-exports function for provides a clean, direct import path,
// and hides the internal module structure from users of the library
pub use commands::construct_command;
pub use commands::resolve_command;
pub use commands::COMMANDS;
pub use constants::AGENTS;
pub use constants::INSTALL_PAGE;
pub use constants::LOCKS;
pub use detect::detect;
pub use detect::get_user_agent;
