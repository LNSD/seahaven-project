/// A trait that converts a command into a [`tokio::process::Command`].
///
/// This trait is used to convert the command into a [`tokio::process::Command`] that can be used to run the command.
pub trait IntoCommand {
    /// Convert the command into a [`tokio::process::Command`].
    fn into_command(self) -> tokio::process::Command;
}

impl IntoCommand for tokio::process::Command {
    fn into_command(self) -> tokio::process::Command {
        self
    }
}

/// A trait that converts a command option into its value.
///
/// This trait is used to convert command options into their values when building the command.
pub trait IntoCmdOptValue<T> {
    /// Convert the option into its value.
    fn into_value(self) -> Option<T>;
}
