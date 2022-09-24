use super::{Expression, Identifier};
use serde::{Deserialize, Serialize};

/// A for expression is a construct for constructing a collection by projecting the items from
/// another collection.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename = "$hcl::for_expr")]
pub enum ForExpr {
    /// Represents a `for` expression that produces a list.
    List(ForListExpr),
    /// Represents a `for` expression that produces an object.
    Object(ForObjectExpr),
}

impl From<ForListExpr> for ForExpr {
    fn from(expr: ForListExpr) -> Self {
        ForExpr::List(expr)
    }
}

impl From<ForObjectExpr> for ForExpr {
    fn from(expr: ForObjectExpr) -> Self {
        ForExpr::Object(expr)
    }
}

/// A `for` expression that produces a list.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename = "$hcl::for_list_expr")]
pub struct ForListExpr {
    /// The "introduction" of the `for` expression.
    pub intro: ForIntro,
    /// An expression that is evaluated once for each element in the source collection.
    pub element_expr: Expression,
    /// An optional filter expression. Elements for which the condition evaluates to `true` will
    /// be evaluated as normal, while if `false` the element will be skipped.
    pub cond_expr: Option<Expression>,
}

impl ForListExpr {
    /// Create a new `ForListExpr` with given intro and an expression that is evaluated once for
    /// each element in the source collection.
    pub fn new<T>(intro: ForIntro, expr: T) -> ForListExpr
    where
        T: Into<Expression>,
    {
        ForListExpr {
            intro,
            element_expr: expr.into(),
            cond_expr: None,
        }
    }

    /// Sets the filter expression. Elements for which the condition evaluates to `true` will be
    /// evaluated as normal, while if `false` the element will be skipped.
    pub fn with_cond_expr<T>(mut self, cond_expr: T) -> ForListExpr
    where
        T: Into<Expression>,
    {
        self.cond_expr = Some(cond_expr.into());
        self
    }
}

/// A `for` expression that produces an object.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename = "$hcl::for_object_expr")]
pub struct ForObjectExpr {
    /// The "introduction" of the `for` expression.
    pub intro: ForIntro,
    /// An expression that is evaluated once for each element key in the source collection.
    pub key_expr: Expression,
    /// An expression that is evaluated once for each element value in the source collection.
    pub value_expr: Expression,
    /// Indicates whether grouping mode is enabled. In grouping mode, each value in the resulting
    /// object is a list of all of the values that were produced against each distinct key.
    pub grouping: bool,
    /// An optional filter expression. Elements for which the condition evaluates to `true` will
    /// be evaluated as normal, while if `false` the element will be skipped.
    pub cond_expr: Option<Expression>,
}

impl ForObjectExpr {
    /// Create a new `ForObjectExpr` with given intro and an two expressions that is evaluated
    /// once for each element's key and value in the source collection.
    pub fn new<K, V>(intro: ForIntro, key_expr: K, value_expr: V) -> ForObjectExpr
    where
        K: Into<Expression>,
        V: Into<Expression>,
    {
        ForObjectExpr {
            intro,
            key_expr: key_expr.into(),
            value_expr: value_expr.into(),
            grouping: false,
            cond_expr: None,
        }
    }

    /// Sets the filter expression. Elements for which the condition evaluates to `true` will be
    /// evaluated as normal, while if `false` the element will be skipped.
    pub fn with_cond_expr<T>(mut self, cond_expr: T) -> ForObjectExpr
    where
        T: Into<Expression>,
    {
        self.cond_expr = Some(cond_expr.into());
        self
    }

    /// Enables or disabled grouping mode.
    pub fn with_grouping(mut self, yes: bool) -> ForObjectExpr {
        self.grouping = yes;
        self
    }
}

/// The `for` keyword followed by either one or two identifiers, the `in` keyword and an
/// expression that must evaluate to a value that can be iterated.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename = "$hcl::for_intro")]
pub struct ForIntro {
    /// Optional name of the variable that will be temporarily assigned the key of each element
    /// during iteration. For lists, this represents the zero-based element index, for objects this
    /// represents the object key.
    pub key_var: Option<Identifier>,
    /// The name of the variable that will be temporarily assigned the value of each element during
    /// iteration.
    pub value_var: Identifier,
    /// An expression that must evaluate to a collection value that can be iterated.
    pub collection_expr: Expression,
}

impl ForIntro {
    /// Creates a new `ForIntro` with the name of the variable that will be temporarily assigned
    /// the value of each element during iteration and an expression that must evaluate to a value
    /// that can be iterated.
    pub fn new<T>(value: Identifier, expr: T) -> ForIntro
    where
        T: Into<Expression>,
    {
        ForIntro {
            key_var: None,
            value_var: value,
            collection_expr: expr.into(),
        }
    }

    /// Adds the iterator key variable identifier to the `for` expression and returns the modified
    /// `ForIntro`.
    pub fn with_key(mut self, key: Identifier) -> ForIntro {
        self.key_var = Some(key);
        self
    }
}
