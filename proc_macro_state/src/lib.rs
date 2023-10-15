//! # macro_state
//! Macro state which can be processed using the [`proc_macro2`](https://docs.rs/proc-macro2/latest/proc_macro2/)
//! library.
//!
//! # Tenets
//! 1. Diagnostics should be testable using Unit Tests
//! 2. TokenStreams should be representable with a `struct`
//! 3.
//!
//! # Usage
//! ## Instantiating State
//! ```rust
//! # use proc_macro2::{Span, TokenStream};
//! # use quote::quote;
//! # use syn::Ident;
//! # use proc_macro_state::{Diagnostic, MacroState, Level, Message};
//!
//! struct ExampleMacroState {
//!     struct_name: Ident
//! }
//!
//! # fn example_macro_input() {
//! intermediate_step(quote!(struct example {}).into());
//! # }
//!
//! fn intermediate_step (tokens: TokenStream) -> MacroState<ExampleMacroState>{
//!     let macro_state = MacroState::new();
//!     let diagnostic = Diagnostic {
//!         span: Span::call_site(),
//!         level: Level::Warning,
//!         message: Message::new("message".to_string())
//!     };
//!     macro_state.extend_diagnostics(vec![ diagnostic ]);
//! }
//!
//! ```
//!
//! ## Expansion into TokenStream
//! This is the final step the MacroState, which is then expanded and emits Diagnostic messages.
//! ```rust
//! # use proc_macro2::{Span, TokenStream};
//! # use quote::quote;
//! # use proc_macro_state::{Diagnostic, Level, MacroState};
//!
//! #[proc_macro]
//! fn example_macro(tokens: TokenStream) -> TokenStream {
//!     let macro_state = MacroState::new(quote!(struct example {}));
//!    #  macro_state.extend_diagnostics(vec![ Diagnostic {
//!    #      span: Span::call_site(),
//!    #      level: Level::Warning,
//!    #      message: "message".parse().unwrap()
//!    #  } ]);
//!     macro_state.expand_and_emit()
//! }
//! ```

lazy_static! {
    pub static ref DIAGNOSTICS: Vec<Diagnostic> = vec![];
}

mod examples;

use std::sync::Mutex;
use derive_builder::Builder;
use lazy_static::lazy_static;
use proc_macro2::Span;
use proc_macro_error::{emit_error, emit_warning};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use proc_macro_error::__export::proc_macro;
use syn::parse::{Parse, Parser, ParseStream};
use syn::parse2;

// TODO: Use proc_macro_error more religiously. Figure out how to get this to work with proc2
pub struct MacroState<T> {
    state: T,
    diagnostics: Vec<Diagnostic>
}

impl<T: ToTokens + Parse> MacroState<T> {
    /// Mutates existing state to update the diagnostics
    pub fn extend_diagnostics<I: IntoIterator<Item = MacroState<T>>>(mut self, state_iterator: I) -> MacroState<T> {
        for state in state_iterator {
            self.diagnostics.extend(state.diagnostics);
        }
        self
    }

    pub fn new(state: T) -> MacroState<T> {
        MacroState {
            state,
            diagnostics: vec![]
        }
    }

    /// Used for the final conversion between `MacroState` and `TokenStream`:
    /// 1. Calls `emit()` for all diagnostics
    pub fn expand_and_emit(self) -> TokenStream {
        for diag in self.diagnostics { diag.emit() }
        self.state.to_token_stream().into()
    }
}

/// Used to convert tokens at intermediate
impl<T: ToTokens + Parse> ToTokens for MacroState<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.state.to_token_stream())
    }
}

#[derive(Clone)]
pub enum Level {
    Error,
    Warning
}

#[derive(Builder)]
pub struct Diagnostic {
    pub span: Span,
    pub level: Level,
    pub message: Message
}

#[derive(Clone)]
pub struct Message {
    pub message: String,
    pub notes: Vec<(String, String)>
}

impl Message {
    pub fn new(message: String) -> Message {
        Message { message, notes: vec![] }
    }
}

impl Diagnostic {
    fn emit(&self) {
        match self.level {
            Level::Error => emit_error!(self.span, self.message),
            Level::Warning => emit_warning!(self.span, self.message),
        }
    }
}

// TODO: test this builder to see if it can be modified even without mutability
impl<'a> From<ParseStream<'a>> for DiagnosticBuilder {
    fn from(parse_stream: ParseStream) -> Self {
        DiagnosticBuilder::default()
            .span(parse_stream.span().into())
            .to_owned()
    }
}






