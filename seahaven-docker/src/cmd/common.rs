/// A trait that converts a command into a [`tokio::process::Command`].
///
/// This trait is used to convert the command into a [`tokio::process::Command`] that can be used to run the command.
pub trait IntoCommand {
    /// Convert the command into a [`tokio::process::Command`].
    fn into_command(self) -> tokio::process::Command;
}
