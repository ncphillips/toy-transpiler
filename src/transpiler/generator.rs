use super::node::{CallNode, DefNode, IntNode, Node, RootNode, VarRefNode};

pub fn generate(node: &Node) -> String {
    match node {
        Node::Root(root_node) => generate_root(root_node),
        Node::Def(def_node) => generate_def(def_node),
        Node::Call(call_node) => generate_call(call_node),
        Node::Int(int_node) => generate_int(int_node),
        Node::VarRef(var_ref_node) => generate_var_ref(var_ref_node),
    }
}

fn generate_root(root_node: &RootNode) -> String {
    let mut body_expr = Vec::new();
    for b in &root_node.body {
        body_expr.push(generate(&b));
    }
    format!("{}", body_expr.join("\n"))
}

fn generate_def(def_node: &DefNode) -> String {
    let body: String;
    let mut body_expr = Vec::new();
    for b in &def_node.body {
        body_expr.push(generate(&b));
    }

    if let Some(final_expression) = body_expr.pop() {
        body_expr.push(format!("return {}", final_expression));
    }


    if body_expr.is_empty() {
        body = String::from("{ }");
    } else {
        body = format!("{{ {} }}", body_expr.join(";"));
    }

    format!(
        "function {}({}) {}",
        def_node.name,
        def_node.arg_names.join(", "),
        body
    )
}

fn generate_call(call_node: &CallNode) -> String {
    let mut arg_expr = Vec::new();
    for expr in &call_node.arg_expr {
        arg_expr.push(generate(&expr));
    }
    format!("{}({})", call_node.name, arg_expr.join(", "))
}

fn generate_int(int_node: &IntNode) -> String {
    format!("{}", int_node.value)
}

fn generate_var_ref(var_ref_node: &VarRefNode) -> String {
    var_ref_node.name.to_string()
}
