use crate::Span;
use leo_ast::common::Identifier as AstIdentifier;

use std::fmt;

/// An identifier in the constrained program.
#[derive(Clone, Hash)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn is_self(&self) -> bool {
        self.name == "Self"
    }
}

impl<'ast> From<AstIdentifier<'ast>> for Identifier {
    fn from(identifier: AstIdentifier<'ast>) -> Self {
        Self {
            name: identifier.value,
            span: Span::from(identifier.span),
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Identifier {}
