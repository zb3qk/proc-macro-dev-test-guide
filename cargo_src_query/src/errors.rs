//! # Cargo Source Query Errors
//!
//! Errors are the primary means of communication that a library can have with its user. Errors
//! can be used as a learning tool with a tight feedback loop, giving users targeted and specific
//! guidance on their interactions with the library. In some ways, Errors are more valuable
//! than static documentation such as this page because of how customizable Error messages can
//! be. With this module, we aim to respect the power that Errors hold for a library and provide
//! as much relevant context as possible to the user.
//!
//! ## Tenets
//! 1. Errors should give the user enough information to root cause issues rooted with
//! synchronization between their input and their larger directory/file ecosystem
//! 2. If an issue needs internal logs to be debugged, those logs should be propagated as a
//! [CargoQueryError] `index_message`
//! 3. Each error should be characterized with a unique top level message that gives users
//! an insight into what the issue may be. This should be capped to 250 characters.
//! 4. Error messages should provide internal developers with enough information to accurately
//! reproduce the issue/identify gaps in knowledge
//!
//! ## Recommendations for Library Dependents
//! 1. Error messages should eventually get passed to a ProcMacroError [1] which can be surfaced by
//! IDEs at compile time.
//! 2. Logs should not be immediately visible to users, but should be easily accessible to
//! provide to internal developers the ability to debug the problem more effectively. A solution
//! to this specification can be found [here]() TODO: Add an example here :)
//!
//! [1] https://docs.rs/proc-macro-error/latest/proc_macro_error/#note-attachments
//!

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use maplit::btreemap;
use proc_macro2::Span;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::PathSegment;

pub fn invalid_crate_name(span: Span, crate_name: &String) {
    proc_macro_error::abort!(
        span, "Invalid crate name.";
        note = "crate name: `{}`", crate_name;
        help = "try including this cargo crate using `cargo add {}`", crate_name;
        help = "or include the cargo crate by following the directions \
        on [crates.io - {}](https://crates.io/crates/{})", crate_name, crate_name;
    )
}

/// Designed to easily convert into a Procedural Macro Error [1]. This error information
/// should be sufficient to allow users to debug their context/environment. Internal
/// debugging tools should come in the form of trace logs.
///
/// [1] https://docs.rs/proc-macro-error/latest/proc_macro_error/#note-attachments
#[derive(Debug)]
pub struct CargoQueryError {
    /// Overarching message displayed to the user to indicate the general problem
    top_level_message: String,
    /// Context clues on what the root cause of the problem may be. These messages
    /// are populated at varying scopes in the lifetime of the library.
    indexed_messages: BTreeMap<String, Vec<String>>,
    /// Backtrace of error generated within the scope of this library
    #[cfg(feature = "backtrace")]
    backtrace: Backtrace
    // TODO: Fix printout for Backtrace. May need to implement Debug/Display for it
    // TODO: Add entry to generate Github issue with (1) function signature (2) logs emitted
    // Idea: Use https://docs.rs/tempfile/latest/tempfile/ to generate a file for each instance? Gets cleaned up when it leaves scope ...
}

pub mod message_identifier {
    pub const SOURCE_PATH: &str = "source path";
    pub const MODULE_PATH: &str = "module path";
}

pub trait AddMessages<T : Debug, E: Error> {
    fn add_messages(self, messages: BTreeMap<&str, String>) -> Self;
}

///
/// ### Syntax
/// ```
/// # use std::io::Error;
/// # use maplit::btreemap;
/// # use cargo_src_query::errors::CargoQueryError;
/// let result: result<String, Error> = Ok("boop");
/// CargoQueryError::from(result).add_messages(btreemap! {
///     "boop": "boop"
/// });
/// ```
// impl<T : Debug, E: Error> AddMessages<T, E> for Result<T, ProcMacroError> {
//     fn add_messages(self, messages: BTreeMap<&str, String>) -> Self {
//         if let Err(mut e) = &self {
//             for (index, message) in messages {
//                 e.indexed_messages.entry(index.to_string()).or_insert(vec![]).push(message);
//             }
//             eprintln!("{self:#?}");
//         }
//         self
//     }
// }

pub trait IntoProcMacroError<T> {
    fn into_proc_err(self) -> Result<T, CargoQueryError>;
}

macro_rules! into_proc_macro_error {
    ($result_type:path) => {
        impl<T> IntoProcMacroError<T> for $result_type {
            fn into_proc_err(self) -> Result<T, CargoQueryError> {
                    match self {
                        Ok(v) => Ok(v),
                        Err(e) => Err(CargoQueryError {
                            top_level_message: e.to_string(),
                            indexed_messages: Default::default(),
                            #[cfg(feature = "backtrace")]
                            backtrace: Backtrace::capture(),
                        })
                    }
            }
        }
    };
}

into_proc_macro_error!{ anyhow::Result<T> }
into_proc_macro_error!{ syn::Result<T> }
into_proc_macro_error!{ std::io::Result<T> }


// TODO: Should these be extension functions on Result<> for conciseness?
impl CargoQueryError {
    // pub fn from<T, E: Error>(value: Result<T, E>) -> Result<T, ProcMacroError> {
    //     match value {
    //         Ok(v) => Ok(v),
    //         Err(e) => Err(ProcMacroError {
    //             span: Span::call_site(),
    //             top_level_message: e.to_string(),
    //             indexed_messages: Default::default(),
    //             backtrace: Backtrace::capture(),
    //         })
    //     }
    // }

      pub fn add_messages(mut self, messages: BTreeMap<&str, String>) -> Self {
        for (index, message) in messages {
            self.indexed_messages.entry(index.to_string()).or_insert(vec![]).push(message);
        }
        eprintln!("{self:#?}");
        self
    }

    pub fn convert_error(description: String, indexed_messages: BTreeMap<String, Vec<String>>) -> CargoQueryError {
        CargoQueryError {
            top_level_message: description,
            indexed_messages,
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }

    /// This Error should enver be used and is only useful for placeholder error messaging
    pub fn generic_error() -> CargoQueryError {
        CargoQueryError {
            top_level_message: "An error has occurred.".to_string(),
            indexed_messages: Default::default(),
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        }
    }

    pub fn could_not_process_lib_rs(error: anyhow::Error) -> CargoQueryError {
        CargoQueryError {
            top_level_message: "Could not process `lib.rs` in src_path.".to_string(),
            indexed_messages: btreemap! {
                "file error".to_string() => vec![error.to_string()],
                "file error backtrace".to_string() => vec![format!("{:#?}", error.backtrace().to_string())]
            },
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }

    pub fn could_not_process_file(file_path: &std::path::Path, error: anyhow::Error) -> CargoQueryError {
        let file_path_string = file_path.to_string_lossy();
        CargoQueryError {
            top_level_message: format!("Could not process `{file_path_string}` in src_path."),
            indexed_messages: btreemap! {
                "file error".to_string() => vec![error.to_string()],
                "file error backtrace".to_string() => vec![format!("{:#?}", error.backtrace().to_string())]
            },
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }

    pub fn could_not_find_defined_module(module: &Ident) -> CargoQueryError {
        CargoQueryError {
            top_level_message: format!("Could not find module `{module}` in scope"),
            indexed_messages: btreemap! {},
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }
    pub fn could_not_find_module_in_scope(module: Ident) -> CargoQueryError {
        CargoQueryError {
            top_level_message: format!("Could not find module `{module}` in scope"),
            indexed_messages: btreemap! {},
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }


    pub fn could_not_find_module_from_path(current_directory: &std::path::Path) -> CargoQueryError {
        let current_directory = current_directory.to_str().unwrap_or("Could not find directory");
        CargoQueryError {
            top_level_message: "Module path does not map to any known module. \
            Using the src_path, manually validate that the module you are looking for exists. \
            If it does exist, cut an issue [here]().".to_string(),
            indexed_messages: btreemap! {
                "current_directory".into() => vec![current_directory.into()]
            },
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }

    pub fn could_not_find_module() -> CargoQueryError {
        CargoQueryError {
            top_level_message: "Module path does not map to any known module. \
            Using the src_path, manually validate that the module you are looking for exists. \
            If it does exist, cut an issue [here]().".to_string(),
            indexed_messages: Default::default(),
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }

    pub fn could_not_find_module_in_file(module: &Ident) -> CargoQueryError {
        CargoQueryError {
            top_level_message: format!("Could not find module `{module}` in source file."),
            indexed_messages: Default::default(),
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture()
        }
    }

    pub fn src_path_to_string(src_path: &std::path::Path) -> String {
        src_path.to_str().unwrap().to_string()
    }

    pub fn module_path_to_string(mod_path: &syn::Path) -> String {
        mod_path.to_token_stream().to_string()
    }
}