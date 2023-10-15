//! ModulePath
//! This is an abstraction layer over the expected interactions with a ModulePath of the form
//! `crate::module_a::module_b::module_c::Definition`.

use std::fmt::{Display, Formatter};
use proc_macro2::Ident;


/// A ModulePath is a path of the form `crate::module_a::module_b::module_c::Definition`. It is
/// used to represent the path of a definition in a crate.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ModulePath {
    path: Vec<Ident>,
    position: usize,
}

impl From<syn::Path> for ModulePath {
    fn from(path: syn::Path) -> Self {
        ModulePath {
            path: path.segments.into_iter().map(|segment| segment.ident).collect(),
            position: 0,
        }
    }
}

impl Display for ModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for (i, ident) in self.path.iter().enumerate().skip(self.position) {
            if i == 0 {
                path.push_str(&ident.to_string());
            } else {
                path.push_str(&format!("::{}", ident));
            }
        }
        write!(f, "{}", path)
    }
}

impl ModulePath {
    /// Create a new ModulePath from a path of the form:
    /// `crate::module_a::module_b::module_c::Definition`.
    ///
    /// ```
    /// # use proc_macro2::Ident;
    /// # use cargo_src_query::helpers::module_path::ModulePath;
    /// #
    /// # fn ident(name: &str) -> Ident {
    /// #   Ident::new(name, proc_macro2::Span::call_site())
    /// # }
    ///
    /// # let a = ident("a"); let b = ident("b"); let c = ident("c");
    /// let mut  module_path = ModulePath::new(vec![a, b, c]);
    ///
    /// assert_eq!(module_path.get_current_module(), Some(ident("a")));
    /// assert_eq!(module_path.next(), Some(ident("b")));
    /// assert_eq!(module_path.get_current_module(), Some(ident("b")));
    /// assert_eq!(module_path.next(), Some(ident("c")));
    /// assert_eq!(module_path.next(), None);
    /// ```
    pub fn new(path: Vec<Ident>) -> Self {
        ModulePath {
            path,
            position: 0,
        }
    }

    /// Returns a new Module Path where current position is overwritten with the provided value.
    /// The original path is not affected.
    ///
    /// `returns` a new ModulePath with the overwritten value.
    ///
    /// ```
    /// # use proc_macro2::Ident;
    /// # use cargo_src_query::helpers::module_path::ModulePath;
    /// #
    /// # fn ident(name: &str) -> Ident {
    /// #   Ident::new(name, proc_macro2::Span::call_site())
    /// # }
    ///
    /// # let a = ident("a"); let b = ident("b"); let c = ident("c");
    ///
    /// let mut module_path = ModulePath::new(vec![a, b]);
    /// let mut  overwritten_module_path = module_path.clone_and_overwrite(c);
    ///
    /// assert_eq!(module_path.get_current_module(), Some(ident("a")));
    /// assert_eq!(overwritten_module_path.get_current_module(), Some(ident("c")));
    ///
    /// // Original path is not affected and renders as `a::b`
    /// assert_eq!(module_path.next(), Some(ident("b")));
    ///
    /// // Inserted path renders as `c::b`
    /// assert_eq!(overwritten_module_path.next(), Some(ident("b")));
    ///
    /// // Module Path does not affect cloned module path
    /// assert_eq!(module_path.next(), None);
    /// assert_eq!(overwritten_module_path.next(), None);
    /// ```
    pub fn clone_and_overwrite(&mut self, overwrite_value: Ident) -> ModulePath {
        let mut cloned_module_path = Self::clone(self);
        cloned_module_path.path[self.position] = overwrite_value;
        cloned_module_path
    }

    /// Returns a new Module Path where current position is overwritten with the provided value.
    /// The original path is not affected.
    ///
    /// ```
    /// # use proc_macro2::Ident;
    /// # use cargo_src_query::helpers::module_path::ModulePath;
    /// #
    /// # fn ident(name: &str) -> Ident {
    /// #   Ident::new(name, proc_macro2::Span::call_site())
    /// # }
    ///
    /// # let a = ident("a"); let b = ident("b"); let c = ident("c");
    ///
    /// let mut module_path = ModulePath::new(vec![a, b]);
    /// let mut  overwritten_module_path = module_path.clone_and_insert(c);
    ///
    /// // Original path is not affected and renders as `a::b`
    /// assert_eq!(module_path.get_current_module(), Some(ident("a")));
    /// assert_eq!(module_path.next(), Some(ident("b")));
    ///
    /// // Inserted path renders as `c::a::b`
    /// assert_eq!(overwritten_module_path.get_current_module(), Some(ident("c")));
    /// assert_eq!(overwritten_module_path.next(), Some(ident("a")));
    /// assert_eq!(overwritten_module_path.next(), Some(ident("b")));
    ///
    /// // Module Path does not affect cloned module path
    /// assert_eq!(module_path.get_current_module(), Some(ident("b")));
    /// assert_eq!(module_path.next(), None);
    ///
    /// ```
    pub fn clone_and_insert(&mut self, insert_value: Ident) -> ModulePath {
        let mut cloned_module_path = Self::clone(self);
        cloned_module_path.path.insert(self.position, insert_value);
        cloned_module_path
    }

    /// Returns the path identifier at the current position.
    ///
    /// ```
    /// # use proc_macro2::Ident;
    /// # use cargo_src_query::helpers::module_path::ModulePath;
    /// #
    /// # fn ident(name: &str) -> Ident {
    /// #   Ident::new(name, proc_macro2::Span::call_site())
    /// # }
    ///
    /// # let a = ident("a"); let b = ident("b"); let c = ident("c");
    ///
    /// let mut module_path = ModulePath::new(vec![a, b, c]);
    ///
    /// // Initial Position
    /// assert_eq!(module_path.get_current_module(), Some(ident("a")));
    ///
    /// // Move to next position
    /// assert_eq!(module_path.next(), Some(ident("b")));
    /// assert_eq!(module_path.get_current_module(), Some(ident("b")));
    /// ```
    pub fn get_current_module(&self) -> Option<Ident> {
        self.path.get(self.position).cloned()
    }

    /// Returns the current position.
    /// ```
    /// # use proc_macro2::Ident;
    /// # use cargo_src_query::helpers::module_path::ModulePath;
    /// #
    /// # fn ident(name: &str) -> Ident {
    /// #   Ident::new(name, proc_macro2::Span::call_site())
    /// # }
    ///
    /// # let a = ident("a"); let b = ident("b"); let c = ident("c");
    ///
    /// let mut module_path = ModulePath::new(vec![a, b, c]);
    ///
    /// // Initial Position
    /// assert_eq!(module_path.get_position(), 0);
    ///
    /// // Move to next position
    /// module_path.next();
    /// assert_eq!(module_path.get_position(), 1);
    /// ```
    pub fn get_position(&self) -> usize {
        self.position
    }
}

impl Iterator for ModulePath {
    type Item = Ident;
    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        self.path.get(self.position).cloned()
    }
}

#[cfg(test)]
mod test {
    use proc_macro2::Ident;
    use crate::helpers::module_path::ModulePath;

    fn ident(name: &str) -> Ident {
        Ident::new(name, proc_macro2::Span::call_site())
    }

    #[test]
    fn overwrite_does_not_affect_original_path() {
        let a = ident("a"); let expected_a = ident("a");
        let b = ident("b"); let expected_b = ident("b");
        let c = ident("c"); let expected_c = ident("c");

        let mut module_path = ModulePath::new(vec![a, b]);
        let mut cloned_module_path = module_path.clone_and_overwrite(c);

        assert_eq!(cloned_module_path.get_current_module(), Some(expected_c.clone()));
        assert_eq!(module_path.get_current_module(), Some(expected_a.clone()));

        assert_eq!(cloned_module_path, ModulePath::new(vec![expected_c, expected_b.clone()]));
        assert_eq!(module_path, ModulePath::new(vec![expected_a, expected_b]));

        // Cloned Module Path does not affect original module path
        cloned_module_path.next();
        assert_eq!(cloned_module_path.get_position(), 1);
        assert_eq!(module_path.get_position(), 0);

        // Module Path does not affect cloned module path
        module_path.next();
        assert_eq!(module_path.get_position(), 1);
        assert_eq!(cloned_module_path.get_position(), 1);
    }

    #[test]
    fn insert_does_not_affect_original_path() {
        let a = ident("a"); let expected_a = ident("a");
        let b = ident("b"); let expected_b = ident("b");
        let c = ident("c"); let expected_c = ident("c");

        let mut module_path = ModulePath::new(vec![a, b]);
        let mut cloned_module_path = module_path.clone_and_insert(c);

        assert_eq!(cloned_module_path.get_current_module(), Some(expected_c.clone()));
        assert_eq!(module_path.get_current_module(), Some(expected_a.clone()));

        assert_eq!(cloned_module_path, ModulePath::new(vec![expected_c, expected_a.clone(), expected_b.clone()]));
        assert_eq!(module_path, ModulePath::new(vec![expected_a, expected_b]));

        // Cloned Module Path does not affect original module path
        cloned_module_path.next();
        assert_eq!(cloned_module_path.get_position(), 1);
        assert_eq!(module_path.get_position(), 0);

        // Module Path does not affect cloned module path
        module_path.next();
        assert_eq!(module_path.get_position(), 1);
        assert_eq!(cloned_module_path.get_position(), 1);
    }

    #[test]
    fn get_position() {

    }

}
