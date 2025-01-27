use crate::modules::ast::{Expr, Stmt};
use std::fmt::Write;

pub fn to_dot(statements: &[Stmt]) -> String {
    let mut s = String::new();

    writeln!(s, "digraph AST {{").unwrap();

    let mut counter = 0;
    let root = new_node("Root", &mut s, &mut counter);
    for stmt in statements {
        let stmt_node = to_dot_stmt(stmt, &mut s, &mut counter);
        writeln!(s, "{} -> {};", root, stmt_node).unwrap();
    }

    writeln!(s, "}}").unwrap();
    s
}

fn to_dot_stmt(stmt: &Stmt, s: &mut String, counter: &mut i32) -> String {
    match stmt {
        Stmt::Print(expr) => {
            let node_name = new_node("Print", s, counter);
            let expr_node = to_dot_expr(expr, s, counter);
            writeln!(s, "{} -> {};", node_name, expr_node).unwrap();
            node_name
        }
        Stmt::Assign(var, expr) => {
            let label = format!("Assign({})", var);
            let node_name = new_node(&label, s, counter);
            let expr_node = to_dot_expr(expr, s, counter);
            writeln!(s, "{} -> {};", node_name, expr_node).unwrap();
            node_name
        }
    }
}

fn to_dot_expr(expr: &Expr, s: &mut String, counter: &mut i32) -> String {
    match expr {
        Expr::Add(lhs, rhs) => {
            let node_name = new_node("Add", s, counter);
            let lhs_node = to_dot_expr(lhs, s, counter);
            let rhs_node = to_dot_expr(rhs, s, counter);

            writeln!(s, "{} -> {};", node_name, lhs_node).unwrap();
            writeln!(s, "{} -> {};", node_name, rhs_node).unwrap();

            node_name
        }
        Expr::Mul(lhs, rhs) => {
            let node_name = new_node("Mul", s, counter);
            let lhs_node = to_dot_expr(lhs, s, counter);
            let rhs_node = to_dot_expr(rhs, s, counter);

            writeln!(s, "{} -> {};", node_name, lhs_node).unwrap();
            writeln!(s, "{} -> {};", node_name, rhs_node).unwrap();

            node_name
        }
        Expr::Num(n) => {
            let label = format!("Num({})", n);
            new_node(&label, s, counter)
        }
        Expr::Var(name) => {
            let label = format!("Var({})", name);
            new_node(&label, s, counter)
        }
    }
}

fn new_node(label: &str, s: &mut String, counter: &mut i32) -> String {
    let node_name = format!("node{}", *counter);
    *counter += 1;

    writeln!(s, "{} [label=\"{}\"];", node_name, label).unwrap();

    node_name
}
