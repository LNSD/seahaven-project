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

/// A trait that converts a command option struct into an optional value.
///
/// This trait is used internally to convert command option structs (like `DetachedSet` or `BuildSet`)
/// into their corresponding values. It allows for a type-safe way to handle optional command line
/// arguments where the presence or absence of a value is encoded in the type system.
pub(super) trait IntoCmdOptValue<T> {
    /// Convert the command option struct into a [`Option<T>`].
    ///
    /// Returns `Some(value)` if the option is set, or `None` if the option is not set.
    fn into_value(self) -> Option<T>;
}

/// A trait that converts a command option struct into a boolean flag value.
///
/// This trait is used internally to convert command option structs into boolean flags.
/// It provides a type-safe way to handle boolean command line arguments where the presence
/// or absence of a value is encoded in the type system.
///
/// This trait is automatically implemented for any type that implements `IntoCmdOptValue<bool>`.
pub(super) trait IntoCmdFlagValue: IntoCmdOptValue<bool> {
    /// Convert the command option struct into a boolean flag value.
    ///
    /// Returns `true` if the option is set to `Some(true)`, or `false` otherwise.
    fn into_flag_value(self) -> bool;
}

impl<T> IntoCmdFlagValue for T
where
    T: IntoCmdOptValue<bool>,
{
    #[inline]
    fn into_flag_value(self) -> bool {
        matches!(self.into_value(), Some(true))
    }
}
