// Посетитель инкапсулирует алгоритм, который работает с гетерогенной коллекцией
// объектов. Это позволяет писать несколько разных алгоритмов для одних и тех же
// данных без необходимости изменять данные (или их основное поведение).
//
// Более того, шаблон посетителя позволяет отделить обход коллекции объектов
// от операций, выполняемых над каждым объектом.

// The data we will visit

mod ast {

    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

// The abstract visitor
mod visit {
    use crate::ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use ast::*;
use visit::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 {
        panic!()
    }

    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::IntLit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}

// Шаблон посетителя полезен везде, где вы хотите применить алгоритм
// к разнородным данным. Если данные однородны, вы можете использовать шаблон,
// подобный итератору. Использование объекта посетителя (а не функционального подхода)
// позволяет посетителю сохранять состояние и, таким образом,
// передавать информацию между узлами.

// Методы visit_* обычно возвращают void (в отличие от примера).
// В этом случае можно выделить код обхода и разделить его между алгоритмами
// (а также предоставить методы noop по умолчанию).
// В Rust обычный способ сделать это — предоставить функции walk_* для каждой базы данных.
// Например,

pub fn walk_expr<T>(visitor: &mut dyn Visitor<T>, e: &Expr) {
    match *e {
        Expr::IntLit(_) => {}
        Expr::Add(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
        Expr::Sub(ref lhs, ref rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
