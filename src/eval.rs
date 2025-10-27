use crate::{make, read::{Atom, Expression}, env::Env, Arena};
use core::ptr::NonNull;

type EvalResult<'arena> = Result<Atom<'arena>, &'static str>;

fn eval_cons<'arena>(arena: &Arena, env: &Env, exprs: &[Expression]) -> EvalResult<'arena> {
    if exprs.len() != 3 {
	return Err("Invalid number of arguments for define");
    }

    let head = eval_expression(arena, env, &exprs[1])?;
    let tail = eval_expression(arena, env, &exprs[2])?;

    Ok(tail)
}

fn eval_define<'arena>(arena: &Arena, env: &Env, exprs: &[Expression]) -> EvalResult<'arena> {
    if exprs.len() != 3 {
	return Err("Invalid number of arguments for define");
    }

    let args = &exprs[1..];
    let sym = &args[0].payload;
    let value_expr = &args[1];

    if let Atom::Symbol{ name } = sym {
	
    } else {
	return Err("Malformed function defintion");
    }

    let value = eval_expression(arena, env, value_expr);

    value
}

fn eval_expression<'arena>(arena: &Arena, env: &Env, expr: &Expression) -> EvalResult<'arena> {
    let mut current_expr = make!(arena, NonNull<Expression>)
	.map(|e| {
	    *e = NonNull::from_ref(expr);
	    e
	});

    loop {
	if let Some(ref atom) = current_expr {
	    match unsafe { atom.as_ref().payload } {
		Atom::List { body } => {
		    let head = &body[0].payload;
		    match head {
			Atom::Define => {
			    return eval_define(arena, env, &body); 
			}
			Atom::Cons => {
			    return eval_cons(arena, env, &body);
			}
			_ => {
			}
		    }
		}
		Atom::True => return Ok(Atom::True),
		_ => {}
	    }
	}
    }
}
