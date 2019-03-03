use std::fmt;

#[derive(Debug)]
pub enum Node<'code> {
    Root(RootNode<'code>),
    Def(DefNode<'code>),
    Int(IntNode),
    Call(CallNode<'code>),
    VarRef(VarRefNode<'code>),
}

/// RootNode
#[derive(Debug)]
pub struct RootNode<'code> {
    pub body: Vec<Node<'code>>,
}


impl<'code> fmt::Display for RootNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<RootNode body={:?}>",
            self.body,
        )
    }
}

/// DefNode
#[derive(Debug)]
pub struct DefNode<'code> {
    pub name: &'code str,
    pub arg_names: Vec<&'code str>,
    pub body: Vec<Node<'code>>,
}

impl<'code> fmt::Display for DefNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<DefNode name='{}' arg_names={:?} body={:?}>",
            self.name, self.arg_names, self.body,
        )
    }
}

/// IntNode
#[derive(Debug)]
pub struct IntNode {
    pub value: i32,
}

impl fmt::Display for IntNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<IntNode value={} >", self.value,)
    }
}

/// CallNode
#[derive(Debug)]
pub struct CallNode<'code> {
    pub name: String,
    pub arg_expr: Vec<Node<'code>>,
}

impl<'code> fmt::Display for CallNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<CallNode name='{}' >", self.name,)
    }
}

/// VarRefNode
#[derive(Debug)]
pub struct VarRefNode<'code> {
    pub name: &'code str,
}

impl<'code> fmt::Display for VarRefNode<'code> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<VarRefNode name='{}' >", self.name,)
    }
}
