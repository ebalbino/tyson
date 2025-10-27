use crate::read::{Atom, Expression};
use std::fmt::{Error, Write};

pub fn print<W: Write>(
    strbuf: &mut W,
    root: &[Expression],
    quoted: bool,
) -> Result<(), Error> {
    for (position, expr) in root.iter().enumerate() {
        let depth = expr.depth;

        if position == 0 && depth > 0 {
            for _ in 0..depth - 1 {
                write!(strbuf, "  ")?;
            }

            if quoted {
                write!(strbuf, "(quote ")?;
            } else {
                write!(strbuf, "(")?;
            }
        }

        if position != 0 {
            write!(strbuf, " ")?;
        }

        match expr.payload {
            Atom::Statement { ref body } => {
                writeln!(strbuf)?;
                print(strbuf, body, false)?;
                write!(strbuf, ")")?;
            }
            Atom::Code { ref body } => {
                writeln!(strbuf)?;
                print(strbuf, body, true)?;
                write!(strbuf, ")")?;
            }
            Atom::Int { inner } => {
                write!(strbuf, "{}", inner)?;
            }
            Atom::Number { inner } => {
                write!(strbuf, "{}", inner)?;
            }
            Atom::False => {
                write!(strbuf, "#f")?;
            }
            Atom::True => {
                write!(strbuf, "#t")?;
            }
            Atom::Void => {
                if quoted {
                    write!(strbuf, "'()")?;
                } else {
                    write!(strbuf, "()")?;
                }
            }
            Atom::Nil => {
                write!(strbuf, "#nil")?;
            }
            Atom::String { inner } => {
                write!(strbuf, "{}", inner)?;
            }
            Atom::Buffer { data } => {
                write!(strbuf, "buffer {}", data.len())?;
            }
            Atom::File { path, lazy } => {
                if lazy {
                    write!(strbuf, "lazyload \"{}\"", path)?;
                } else {
                    write!(strbuf, "load \"{}\"", path)?;
                }
            }
            Atom::Define => {
                write!(strbuf, "define")?;
            }
            Atom::Cons => {
                write!(strbuf, "cons")?;
            }
            Atom::Head => {
                write!(strbuf, "head")?;
            }
            Atom::Tail => {
                write!(strbuf, "tail")?;
            }
            Atom::Binding { name } => {
                write!(strbuf, "{name}")?;
            }
            Atom::Add => {
                write!(strbuf, "+")?;
            }
            Atom::Subtract => {
                write!(strbuf, "-")?;
            }
            Atom::Multiply => {
                write!(strbuf, "*")?;
            }
            Atom::Divide => {
                write!(strbuf, "/")?;
            }
            Atom::Eq => {
                write!(strbuf, "=")?;
            }
            Atom::Neq => {
                write!(strbuf, "!=")?;
            }
            Atom::GT => {
                write!(strbuf, ">")?;
            }
            Atom::LT => {
                write!(strbuf, "<")?;
            }
            Atom::GTE => {
                write!(strbuf, ">=")?;
            }
            Atom::LTE => {
                write!(strbuf, "<=")?;
            }
            Atom::ArrowLeft => {
                write!(strbuf, "->")?;
            }
            Atom::ArrowRight => {
                write!(strbuf, "<-")?;
            }
            Atom::Negate => {
                write!(strbuf, "!")?;
            }
            Atom::Exp => {
                write!(strbuf, "**")?;
            }
            Atom::Mod => {
                write!(strbuf, "%")?;
            }
            Atom::Remainder => {
                write!(strbuf, "//")?;
            }
        }
    }

    Ok(())
}
