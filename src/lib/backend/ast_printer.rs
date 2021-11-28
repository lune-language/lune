use super::ast::*;

pub struct ASTPrinter {

}

impl ASTPrinter {
    /// Print out the AST joined together with newlines
    pub fn print(&mut self, stmts: Vec<Stmt>) {
        println!("{}\n", "AST print (S expressions):");
        println!("[");
        for (pos, stmt) in stmts.iter().enumerate() {
            if pos > 0 {
                println!(" {}", self.visit_stmt(&stmt));
            }
            else {
                println!("{}", self.visit_stmt(&stmt));
            }
        }
        println!("]");
    }
}

impl Visitor<String> for ASTPrinter {
    fn visit_name(&mut self, name: &Name) -> String {
        name.value.clone()
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> String {
        match &*stmt {
            Stmt::VarDeclaration(name, _, value) => format!("(var {} {})", name.value.clone(), self.visit_expr(value)),
            _ => "undefined".into()
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> String {
        match &*expr {
            Expr::IntLit(n) => n.to_string(),
            Expr::StringLit(s) => format!("\"{}\"", s),

            Expr::BinOp(lhs, op, rhs) => format!("({} {} {})", op, lhs.clone(), rhs.clone()),
            Expr::UnaryOp(op, rhs) => format!("({} {})", op, rhs.clone())
        }
    }
}