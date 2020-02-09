use crate::parse::AST;
use crate::Type;
use std::collections::HashMap;

pub fn tc(ast: Box<AST>, tenv: &HashMap<String, Type>) -> Type {
    match *ast {
        AST::Anumc => Type::NumT,
        AST::AtrueC => Type::BoolT,
        AST::AfalseC => Type::BoolT,
        AST::AplusC(op1, op2) => {
        	if tc(op1, &tenv) == Type::NumT && tc(op2, &tenv) == Type::NumT {
        		Type::NumT;
        	}
        	panic!("Types differ in AplusC!")
        }
        AST::AmultC(op1, op2) => {
        	if tc(op1, &tenv) == Type::NumT && tc(op2, &tenv) == Type::NumT {
        		Type::NumT;
        	}
        	panic!("Types differ in AmultC!")
        }
        AST::AifC {cnd, then, els} => {
        	let cnd_type = tc(cnd, tenv);
        	if cnd_type != Type::BoolT {
        		panic!("Condition in an if statement is not boolean!")
        	}
        	else {
        		let then_type = tc(then, tenv);
        		let else_type = tc(els, tenv);
        		if then_type == else_type {
        			then_type
        		}
        		else {
        			panic!("Types differ in then and else part of an if statement!")
        		}
        	}
        }
        AST::AidC(id) => {
        	if tenv.contains_key(&id) {
        		tenv[&id].clone()
        	}
        	else {
        		panic!("Variable not saved in type environment")
        	}
        }
        // AST::AappC {func, arg} => {

        // }
        _ => Type::NumT,
    }

    // unimplemented!()
}
