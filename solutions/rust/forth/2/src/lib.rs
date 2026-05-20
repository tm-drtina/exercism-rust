use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
type Op = Rc<dyn Fn(&mut Vec<Value>) -> Result>;
pub type Result = std::result::Result<(), Error>;

enum Item {
    Value(Value),
    Op(Op),
}

pub struct Forth{
    stack: Vec<Value>,
    ops: HashMap<String, Op>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        let mut ops = HashMap::<String, Op>::new();

        ops.insert(String::from("+"), Rc::new(|stack: &mut Vec<Value>| {
            let n2 = stack.pop().ok_or(Error::StackUnderflow)?;
            let n1 = stack.pop().ok_or(Error::StackUnderflow)?;
            stack.push(n1 + n2);
            Ok(())
        }));
        ops.insert(String::from("-"), Rc::new(|stack: &mut Vec<Value>| {
            let n2 = stack.pop().ok_or(Error::StackUnderflow)?;
            let n1 = stack.pop().ok_or(Error::StackUnderflow)?;
            stack.push(n1 - n2);
            Ok(())
        }));
        ops.insert(String::from("*"), Rc::new(|stack: &mut Vec<Value>| {
            let n2 = stack.pop().ok_or(Error::StackUnderflow)?;
            let n1 = stack.pop().ok_or(Error::StackUnderflow)?;
            stack.push(n1 * n2);
            Ok(())
        }));
        ops.insert(String::from("/"), Rc::new(|stack: &mut Vec<Value>| {
            let n2 = stack.pop().ok_or(Error::StackUnderflow)?;
            let n1 = stack.pop().ok_or(Error::StackUnderflow)?;
            if n2 == 0 {
                return Err(Error::DivisionByZero);
            }
            stack.push(n1 / n2);
            Ok(())
        }));

        ops.insert(String::from("dup"), Rc::new(|stack: &mut Vec<Value>| {
            let n = *stack.last().ok_or(Error::StackUnderflow)?;
            stack.push(n);
            Ok(())
        }));
        ops.insert(String::from("drop"), Rc::new(|stack: &mut Vec<Value>| {
            stack.pop().ok_or(Error::StackUnderflow)?;
            Ok(())
        }));
        ops.insert(String::from("swap"), Rc::new(|stack: &mut Vec<Value>| {
            let n2 = stack.pop().ok_or(Error::StackUnderflow)?;
            let n1 = stack.pop().ok_or(Error::StackUnderflow)?;
            stack.push(n2);
            stack.push(n1);
            Ok(())
        }));
        ops.insert(String::from("over"), Rc::new(|stack: &mut Vec<Value>| {
            let n2 = stack.pop().ok_or(Error::StackUnderflow)?;
            let n1 = *stack.last().ok_or(Error::StackUnderflow)?;
            stack.push(n2);
            stack.push(n1);
            Ok(())
        }));

        Self {
            stack: Vec::new(),
            ops,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn parse(&self, expr: &str) -> std::result::Result<Vec<Item>, Error> {
        let mut res = Vec::new();
        for word in expr.split_ascii_whitespace() {
            let item = if let Ok(v) = word.parse::<Value>() {
                Item::Value(v)
            } else {
                let op = self.ops.get(&word.to_lowercase()).ok_or(Error::UnknownWord)?;
                Item::Op(Rc::clone(op))
            };
            res.push(item);
        }
        Ok(res)
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if let Some(s) = input.strip_prefix(": ") {
            let def = s.strip_suffix(" ;").expect("invalid syntax");

            let (name, expr) = def.split_once(' ').expect("Invalid syntax");
            if name.parse::<Value>().is_ok() {
                return Err(Error::InvalidWord);
            }
            let parsed = self.parse(expr)?;
            let op = Rc::new(move |stack: &mut Vec<Value>| {
                for item in &parsed {
                    match item {
                        Item::Value(v) => stack.push(*v),
                        Item::Op(op) => op(stack)?,
                    }
                }
                Ok(())
            });
            self.ops.insert(name.to_lowercase(), op);
        } else {
            let parsed = self.parse(input)?;
            for item in parsed {
                match item {
                    Item::Value(v) => self.stack.push(v),
                    Item::Op(op) => op(&mut self.stack)?,
                }
            }
        }
        Ok(())
    }
}
