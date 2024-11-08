// DESCRIPTION
//
// Шаблон проектирования «Стратегия» — это метод, позволяющий разделить задачи.
// Это также позволяет отделять программные модули
// посредством инверсии зависимостей (Dependency Inversion).
//
// Основная идея шаблона «Стратегия» заключается в том, что, учитывая алгоритм,
// решающий конкретную проблему, мы определяем только скелет алгоритма
// на абстрактном уровне и разделяем реализацию конкретного алгоритма на разные части.
//
// Таким образом, клиент, использующий алгоритм, может выбрать конкретную реализацию,
// при этом общий рабочий процесс алгоритма остается прежним.
// Другими словами, абстрактная спецификация класса не зависит от
// конкретной реализации производного класса, но конкретная реализация
// должна соответствовать абстрактной спецификации.
// Вот почему мы называем это «инверсией зависимостей» (Dependency Inversion).

// MOTIVATION
//
// Представьте, что мы работаем над проектом, который генерирует отчеты каждый месяц.
// Нам нужно, чтобы отчеты формировались в разных форматах (стратегиях), например,
// в форматах JSON или Plain Text. Но со временем ситуация меняется, и мы не знаем,
// какие требования мы можем получить в будущем. Например,
// нам может потребоваться создать отчет в совершенно новом формате
// или просто изменить один из существующих форматов.

// In this example our invariants (or abstractions) are Formatter and Report,
// while Text and Json are our strategy structs.
// These strategies have to implement the Formatter trait.

use std::collections::HashMap;

type Data = HashMap<String, u32>;

trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

struct Report;

impl Report {
    // Write should be used but we kept it as String to ignore error handling
    fn generate<T: Formatter>(g: T, s: &mut String) {
        // backend operations...
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        // generate report
        g.format(&data, s);
    }

    pub fn generate_closures<F>(s: &mut String, f: &mut F)
    where
        F: FnMut(&mut String, &Data),
    {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        f(s, &data)
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{k} {v}\n");
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop(); // remove extra, at the end
        }
        buf.push(']');
    }
}

fn main() {
    let mut s = String::from("");
    Report::generate(Text, &mut s);
    println!("generated 'Text': \n{s}");
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear(); // reuse the same buffer
    Report::generate(Json, &mut s);
    println!("generated 'Json': \n{s}");
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));

    s.clear();

    let mut text = |buf: &mut String, data: &Data| {
        for (k, v) in data {
            let entry = format!("{k} {v}\n");
            buf.push_str(&entry);
        }
    };
    
    let mut json = |buf: &mut String, data: &Data| {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop(); // remove extra, at the end
        }
        buf.push(']');
    };
    
    println!("\n With closure....\n");

    Report::generate_closures(&mut s, &mut text);
    println!("generated 'Text': \n{s}");
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear();
    Report::generate_closures(&mut s, &mut json);
    println!("generated 'Json': \n{s}");
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
