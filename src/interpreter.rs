// ArcScript interpreter (MVP subset with basic functions)

use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, FuncDecl, Literal, Program, Stmt, UnaryOp};

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub line: Option<usize>,
}

impl RuntimeError {
    pub fn new(msg: &str) -> Self {
        Self { 
            message: msg.to_string(),
            line: None,
        }
    }
    
    pub fn with_line(msg: &str, line: usize) -> Self {
        Self {
            message: msg.to_string(),
            line: Some(line),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line) = self.line {
            write!(f, "Line {}: {}", line, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for RuntimeError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Function {
        decl: FuncDecl,
        closure: Option<Box<Environment>>,
    },
    Table(HashMap<String, Value>),
    BuiltinFunction(String), // Built-in function by name
    Nil,
}

#[derive(Clone, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), parent: None }
    }

    pub fn with_parent(parent: Option<Box<Environment>>) -> Self {
        Self { values: HashMap::new(), parent }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(v) = self.values.get(name) {
            Some(v)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        // Register built-in functions
        Self::register_builtins(&mut env);
        Self { env }
    }

    fn register_builtins(env: &mut Environment) {
        // Output functions
        env.define("print".to_string(), Value::BuiltinFunction("print".to_string()));
        env.define("println".to_string(), Value::BuiltinFunction("println".to_string()));
        
        // Type operations
        env.define("type".to_string(), Value::BuiltinFunction("type".to_string()));
        env.define("len".to_string(), Value::BuiltinFunction("len".to_string()));
        
        // Type conversions
        env.define("str".to_string(), Value::BuiltinFunction("str".to_string()));
        env.define("int".to_string(), Value::BuiltinFunction("int".to_string()));
        env.define("float".to_string(), Value::BuiltinFunction("float".to_string()));
        
        // Math functions
        env.define("abs".to_string(), Value::BuiltinFunction("abs".to_string()));
        env.define("min".to_string(), Value::BuiltinFunction("min".to_string()));
        env.define("max".to_string(), Value::BuiltinFunction("max".to_string()));
        env.define("floor".to_string(), Value::BuiltinFunction("floor".to_string()));
        env.define("ceil".to_string(), Value::BuiltinFunction("ceil".to_string()));
        env.define("round".to_string(), Value::BuiltinFunction("round".to_string()));
        env.define("sqrt".to_string(), Value::BuiltinFunction("sqrt".to_string()));
        env.define("pow".to_string(), Value::BuiltinFunction("pow".to_string()));
        
        // String functions
        env.define("substring".to_string(), Value::BuiltinFunction("substring".to_string()));
        env.define("contains".to_string(), Value::BuiltinFunction("contains".to_string()));
        env.define("toUpper".to_string(), Value::BuiltinFunction("toUpper".to_string()));
        env.define("toLower".to_string(), Value::BuiltinFunction("toLower".to_string()));
    }

    fn eval_function_body(&mut self, body: &Stmt) -> Result<Option<Value>, RuntimeError> {
        match body {
            Stmt::Block(stmts) => {
                for s in stmts {
                    match self.eval_stmt(s) {
                        Ok(Some(v)) => return Ok(Some(v)),
                        Ok(None) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(None)
            }
            _ => self.eval_stmt(body),
        }
    }

    fn eval_call(&mut self, callee: &Expr, args: &[Expr]) -> Result<Value, RuntimeError> {
        let callee_val = self.eval_expr(callee)?;
        
        // Handle built-in functions
        if let Value::BuiltinFunction(name) = callee_val {
            return self.call_builtin(&name, args);
        }
        
        let (func, closure) = match callee_val {
            Value::Function { decl, closure } => (decl, closure),
            _ => {
                return Err(RuntimeError::new("attempt to call non-function"));
            }
        };

        // Create new environment: if function has closure, chain to that; otherwise chain to current env
        let parent = if let Some(captured) = closure {
            Some(captured)
        } else {
            Some(Box::new(self.env.clone()))
        };
        let mut call_env = Environment::with_parent(parent);

        // Bind parameters to argument values (extra args ignored, missing args become Nil)
        for (i, param) in func.params.iter().enumerate() {
            let value = if let Some(arg_expr) = args.get(i) {
                self.eval_expr(arg_expr)?
            } else {
                Value::Nil
            };
            call_env.define(param.name.clone(), value);
        }

        // Swap in call environment, execute, and restore previous environment
        let saved_env = std::mem::replace(&mut self.env, call_env);
        let result_opt = self.eval_function_body(&func.body)?;
        self.env = saved_env;
        Ok(result_opt.unwrap_or(Value::Nil))
    }

    fn call_builtin(&mut self, name: &str, args: &[Expr]) -> Result<Value, RuntimeError> {
        match name {
            "print" => {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", self.value_to_string(&self.eval_expr(arg)?));
                }
                Ok(Value::Nil)
            }
            "println" => {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", self.value_to_string(&self.eval_expr(arg)?));
                }
                println!();
                Ok(Value::Nil)
            }
            "type" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("type() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                let type_name = match val {
                    Value::Int(_) => "int",
                    Value::Float(_) => "float",
                    Value::Bool(_) => "bool",
                    Value::String(_) => "string",
                    Value::Function { .. } => "function",
                    Value::Table(_) => "table",
                    Value::BuiltinFunction(_) => "builtin_function",
                    Value::Nil => "nil",
                };
                Ok(Value::String(type_name.to_string()))
            }
            "len" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("len() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::Int(s.len() as i64)),
                    Value::Table(t) => Ok(Value::Int(t.len() as i64)),
                    _ => Err(RuntimeError::new("len() requires string or table argument")),
                }
            }
            "str" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("str() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                Ok(Value::String(self.value_to_string(&val)))
            }
            "int" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("int() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Int(i) => Ok(Value::Int(i)),
                    Value::Float(f) => Ok(Value::Int(f as i64)),
                    Value::String(s) => {
                        s.parse::<i64>()
                            .map(Value::Int)
                            .map_err(|_| RuntimeError::new("cannot convert string to int"))
                    }
                    Value::Bool(b) => Ok(Value::Int(if b { 1 } else { 0 })),
                    _ => Err(RuntimeError::new("cannot convert to int")),
                }
            }
            "float" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("float() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Int(i) => Ok(Value::Float(i as f64)),
                    Value::Float(f) => Ok(Value::Float(f)),
                    Value::String(s) => {
                        s.parse::<f64>()
                            .map(Value::Float)
                            .map_err(|_| RuntimeError::new("cannot convert string to float"))
                    }
                    _ => Err(RuntimeError::new("cannot convert to float")),
                }
            }
            // Math functions
            "abs" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("abs() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Int(i) => Ok(Value::Int(i.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err(RuntimeError::new("abs() requires numeric argument")),
                }
            }
            "min" => {
                if args.len() < 2 {
                    return Err(RuntimeError::new("min() requires 2 arguments"));
                }
                let a = self.eval_expr(&args[0])?;
                let b = self.eval_expr(&args[1])?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x.min(y))),
                    (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.min(y))),
                    (Value::Int(x), Value::Float(y)) => Ok(Value::Float((x as f64).min(y))),
                    (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x.min(y as f64))),
                    _ => Err(RuntimeError::new("min() requires numeric arguments")),
                }
            }
            "max" => {
                if args.len() < 2 {
                    return Err(RuntimeError::new("max() requires 2 arguments"));
                }
                let a = self.eval_expr(&args[0])?;
                let b = self.eval_expr(&args[1])?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x.max(y))),
                    (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x.max(y))),
                    (Value::Int(x), Value::Float(y)) => Ok(Value::Float((x as f64).max(y))),
                    (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x.max(y as f64))),
                    _ => Err(RuntimeError::new("max() requires numeric arguments")),
                }
            }
            "floor" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("floor() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
                    Value::Int(i) => Ok(Value::Int(i)),
                    _ => Err(RuntimeError::new("floor() requires numeric argument")),
                }
            }
            "ceil" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("ceil() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
                    Value::Int(i) => Ok(Value::Int(i)),
                    _ => Err(RuntimeError::new("ceil() requires numeric argument")),
                }
            }
            "round" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("round() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Float(f) => Ok(Value::Int(f.round() as i64)),
                    Value::Int(i) => Ok(Value::Int(i)),
                    _ => Err(RuntimeError::new("round() requires numeric argument")),
                }
            }
            "sqrt" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("sqrt() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Float(f) => {
                        if f < 0.0 {
                            Err(RuntimeError::new("sqrt() requires non-negative argument"))
                        } else {
                            Ok(Value::Float(f.sqrt()))
                        }
                    }
                    Value::Int(i) => {
                        if i < 0 {
                            Err(RuntimeError::new("sqrt() requires non-negative argument"))
                        } else {
                            Ok(Value::Float((i as f64).sqrt()))
                        }
                    }
                    _ => Err(RuntimeError::new("sqrt() requires numeric argument")),
                }
            }
            "pow" => {
                if args.len() < 2 {
                    return Err(RuntimeError::new("pow() requires 2 arguments"));
                }
                let base = self.eval_expr(&args[0])?;
                let exp = self.eval_expr(&args[1])?;
                match (base, exp) {
                    (Value::Float(b), Value::Float(e)) => Ok(Value::Float(b.powf(e))),
                    (Value::Float(b), Value::Int(e)) => Ok(Value::Float(b.powi(e as i32))),
                    (Value::Int(b), Value::Float(e)) => Ok(Value::Float((b as f64).powf(e))),
                    (Value::Int(b), Value::Int(e)) => {
                        if e >= 0 && e <= i32::MAX as i64 {
                            Ok(Value::Float((b as f64).powi(e as i32)))
                        } else {
                            Ok(Value::Float((b as f64).powf(e as f64)))
                        }
                    }
                    _ => Err(RuntimeError::new("pow() requires numeric arguments")),
                }
            }
            // String functions
            "substring" => {
                if args.len() < 3 {
                    return Err(RuntimeError::new("substring() requires 3 arguments (string, start, end)"));
                }
                let s = self.eval_expr(&args[0])?;
                let start = self.eval_expr(&args[1])?;
                let end = self.eval_expr(&args[2])?;
                
                match (s, start, end) {
                    (Value::String(s), Value::Int(start), Value::Int(end)) => {
                        let start = start.max(0) as usize;
                        let end = end.max(0) as usize;
                        let chars: Vec<char> = s.chars().collect();
                        let end = end.min(chars.len());
                        let start = start.min(end);
                        Ok(Value::String(chars[start..end].iter().collect()))
                    }
                    _ => Err(RuntimeError::new("substring() requires (string, int, int)")),
                }
            }
            "contains" => {
                if args.len() < 2 {
                    return Err(RuntimeError::new("contains() requires 2 arguments"));
                }
                let s = self.eval_expr(&args[0])?;
                let substr = self.eval_expr(&args[1])?;
                
                match (s, substr) {
                    (Value::String(s), Value::String(substr)) => {
                        Ok(Value::Bool(s.contains(&substr)))
                    }
                    _ => Err(RuntimeError::new("contains() requires two string arguments")),
                }
            }
            "toUpper" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("toUpper() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::String(s.to_uppercase())),
                    _ => Err(RuntimeError::new("toUpper() requires string argument")),
                }
            }
            "toLower" => {
                if args.is_empty() {
                    return Err(RuntimeError::new("toLower() requires 1 argument"));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::String(s.to_lowercase())),
                    _ => Err(RuntimeError::new("toLower() requires string argument")),
                }
            }
            _ => Err(RuntimeError::new(&format!("unknown built-in function: {}", name))),
        }
    }

    fn value_to_string(&self, val: &Value) -> String {
        match val {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Function { .. } => "<function>".to_string(),
            Value::Table(_) => "<table>".to_string(),
            Value::BuiltinFunction(name) => format!("<builtin: {}>", name),
            Value::Nil => "nil".to_string(),
        }
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<(), RuntimeError> {
        for stmt in &program.body {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, RuntimeError> {
        match stmt {
            Stmt::VarDecl { name, init } => {
                let v = self.eval_expr(init)?;
                self.env.define(name.clone(), v);
                Ok(None)
            }
            Stmt::Assignment { name, value } => {
                let v = self.eval_expr(value)?;
                // For assignment, we update existing variable (or create if not exists)
                self.env.define(name.clone(), v);
                Ok(None)
            }
            Stmt::Expr(expr) => {
                let _ = self.eval_expr(expr)?;
                Ok(None)
            }
            Stmt::Block(stmts) => {
                // Create a new scope for the block that chains to the current environment
                let prev_env = self.env.clone();
                let block_env = Environment::with_parent(Some(Box::new(prev_env.clone())));
                let saved_env = std::mem::replace(&mut self.env, block_env);
                for s in stmts {
                    match self.eval_stmt(s) {
                        Ok(Some(v)) => { self.env = saved_env; return Ok(Some(v)); }
                        Ok(None) => continue,
                        Err(e) => { self.env = saved_env; return Err(e); }
                    }
                }
                // restore previous environment
                self.env = saved_env;
                Ok(None)
            }
            Stmt::If { condition, then_branch, elif_branches, else_branch } => {
                if self.truthy(&self.eval_expr(condition)?) {
                    self.eval_stmt(then_branch)?;
                } else {
                    let mut handled = false;
                    for (cond, block) in elif_branches {
                        if self.truthy(&self.eval_expr(cond)?) {
                            self.eval_stmt(block)?;
                            handled = true;
                            break;
                        }
                    }
                    if !handled {
                        if let Some(else_b) = else_branch {
                            self.eval_stmt(else_b)?;
                        }
                    }
                }
                Ok(None)
            }
            Stmt::While { condition, body } => {
                while self.truthy(&self.eval_expr(condition)?) {
                    match self.eval_stmt(body)? {
                        Some(Value::Nil) if matches!(body.as_ref(), Stmt::Break) => break,
                        Some(v) => return Ok(Some(v)), // return from function
                        None => continue,
                    }
                }
                Ok(None)
            }
            Stmt::For { var_name, start, end, step, body } => {
                let start_val = self.eval_expr(start)?;
                let end_val = self.eval_expr(end)?;
                let step_val = if let Some(step_expr) = step {
                    self.eval_expr(step_expr)?
                } else {
                    Value::Int(1)
                };
                
                // Extract numeric values
                let (start_num, end_num, step_num) = match (start_val, end_val, step_val) {
                    (Value::Int(s), Value::Int(e), Value::Int(st)) => (s, e, st),
                    (Value::Float(s), Value::Float(e), Value::Float(st)) => {
                        (s as i64, e as i64, st as i64)
                    }
                    (Value::Int(s), Value::Float(e), Value::Int(st)) => (s, e as i64, st),
                    (Value::Float(s), Value::Int(e), Value::Float(st)) => (s as i64, e, st as i64),
                    _ => return Err(RuntimeError::new("for loop requires numeric start, end, and step")),
                };
                
                if step_num == 0 {
                    return Err(RuntimeError::new("for loop step cannot be zero"));
                }
                
                // Create new scope for the loop
                let prev_env = self.env.clone();
                let loop_env = Environment::with_parent(Some(Box::new(prev_env.clone())));
                let saved_env = std::mem::replace(&mut self.env, loop_env);
                
                let mut i = start_num;
                let result = loop {
                    // Check loop condition based on step direction
                    let should_continue = if step_num > 0 {
                        i <= end_num
                    } else {
                        i >= end_num
                    };
                    
                    if !should_continue {
                        break Ok(None);
                    }
                    
                    // Define/update loop variable
                    self.env.define(var_name.clone(), Value::Int(i));
                    
                    // Execute body
                    match self.eval_stmt(body)? {
                        Some(v) if matches!(body.as_ref(), Stmt::Break) => break Ok(None),
                        Some(v) => break Ok(Some(v)), // return from function
                        None => {}
                    }
                    
                    i += step_num;
                };
                
                self.env = saved_env;
                result
            }
            Stmt::Break => {
                // Break is handled by the loop that contains it
                Ok(Some(Value::Nil))
            }
            Stmt::Continue => {
                // Continue is handled by the loop that contains it
                Ok(None)
            }
            Stmt::Return(expr_opt) => {
                if let Some(e) = expr_opt {
                    let v = self.eval_expr(e)?;
                    Ok(Some(v))
                } else {
                    Ok(Some(Value::Nil))
                }
            }
            Stmt::FuncDecl(func) => {
                // Capture current environment when defining the function (closure)
                let closure = Some(Box::new(self.env.clone()));
                self.env.define(
                    func.name.clone(),
                    Value::Function {
                        decl: func.clone(),
                        closure,
                    },
                );
                Ok(None)
            }
            Stmt::ObjectDecl(obj) => {
                // Create a table for the object with its members
                let mut table = HashMap::new();
                for member in &obj.members {
                    match member {
                        crate::ast::ObjectMember::Var(var_stmt) => {
                            if let Stmt::VarDecl { name, init } = var_stmt {
                                let val = self.eval_expr(init)?;
                                table.insert(name.clone(), val);
                            }
                        }
                        crate::ast::ObjectMember::Method(func) => {
                            // Methods also capture environment as closures
                            let closure = Some(Box::new(self.env.clone()));
                            table.insert(
                                func.name.clone(),
                                Value::Function {
                                    decl: func.clone(),
                                    closure,
                                },
                            );
                        }
                        crate::ast::ObjectMember::Event(_event) => {
                            // Events not yet implemented; skip for now
                        }
                    }
                }
                self.env.define(obj.name.clone(), Value::Table(table));
                Ok(None)
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(lit) => Ok(self.eval_literal(lit)),
            Expr::Ident(name) => {
                if let Some(v) = self.env.get(name) {
                    Ok(v.clone())
                } else {
                    Err(RuntimeError::new(&format!("Undefined identifier '{}'", name)))
                }
            }
            Expr::Unary { op, expr } => {
                let v = self.eval_expr(expr)?;
                match op {
                    UnaryOp::Negate => match v {
                        Value::Int(i) => Ok(Value::Int(-i)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err(RuntimeError::new("type error: unary - on non-number")),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!self.truthy(&v))),
                }
            }
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.apply_binary(op, l, r)
            }
            Expr::Call { callee, args } => {
                self.eval_call(callee, args)
            }
            Expr::Member { object, field } => {
                let obj_val = self.eval_expr(object)?;
                match obj_val {
                    Value::Table(map) => {
                        Ok(map.get(field).cloned().unwrap_or(Value::Nil))
                    }
                    _ => Err(RuntimeError::new(&format!("cannot access member '{}' on non-table", field))),
                }
            }
            Expr::Index { object, index } => {
                let obj_val = self.eval_expr(object)?;
                let index_val = self.eval_expr(index)?;
                match (obj_val, index_val) {
                    (Value::Table(map), Value::String(key)) => {
                        Ok(map.get(&key).cloned().unwrap_or(Value::Nil))
                    }
                    (Value::Table(_), _) => Err(RuntimeError::new("table index must be a string")),
                    _ => Err(RuntimeError::new("cannot index non-table")),
                }
            }
            Expr::TableLiteral(fields) => {
                let mut map = HashMap::new();
                for (idx, field) in fields.iter().enumerate() {
                    match field {
                        crate::ast::TableField::KeyValue { key, value } => {
                            let val = self.eval_expr(value)?;
                            map.insert(key.clone(), val);
                        }
                        crate::ast::TableField::Value(expr) => {
                            let val = self.eval_expr(expr)?;
                            map.insert(idx.to_string(), val);
                        }
                    }
                }
                Ok(Value::Table(map))
            }
        }
    }

    fn eval_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Int(i) => Value::Int(*i),
            Literal::Float(f) => Value::Float(*f),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Nil => Value::Nil,
        }
    }

    fn truthy(&self, v: &Value) -> bool {
        match v {
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    fn negate(&self, v: Value) -> Value {
        match v {
            Value::Int(i) => Value::Int(-i),
            Value::Float(f) => Value::Float(-f),
            _ => Value::Nil,
        }
    }

    fn apply_binary(&self, op: &BinaryOp, left: Value, right: Value) -> Result<Value, RuntimeError> {
        use BinaryOp::*;
        match op {
            Add => self.add(left, right),
            Sub => self.sub(left, right),
            Mul => self.mul(left, right),
            Div => self.div(left, right),
            Mod => self.modulo(left, right),
            Equal => Ok(Value::Bool(self.eq(left, right))),
            NotEqual => Ok(Value::Bool(!self.eq(left, right))),
            Less => self.cmp(|a, b| a < b, left, right),
            LessEqual => self.cmp(|a, b| a <= b, left, right),
            Greater => self.cmp(|a, b| a > b, left, right),
            GreaterEqual => self.cmp(|a, b| a >= b, left, right),
            And => Ok(self.apply_and(left, right)),
            Or => Ok(self.apply_or(left, right)),
        }
    }
            GreaterEqual => self.cmp(|a, b| a >= b, left, right),
            And => Ok(Value::Bool(self.truthy(&left) && self.truthy(&right))),
            Or => Ok(Value::Bool(self.truthy(&left) || self.truthy(&right))),
        }
    }

    fn add(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            _ => Err(RuntimeError::new("type error: cannot add the given operands")),
        }
    }

    fn sub(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
            _ => Err(RuntimeError::new("type error: cannot subtract the given operands")),
        }
    }

    fn mul(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
            _ => Err(RuntimeError::new("type error: cannot multiply the given operands")),
        }
    }

    fn div(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(RuntimeError::new("division by zero"))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if b == 0.0 { Err(RuntimeError::new("division by zero")) } else { Ok(Value::Float(a / b)) }
            }
            (Value::Int(a), Value::Float(b)) => {
                if b == 0.0 { Err(RuntimeError::new("division by zero")) } else { Ok(Value::Float(a as f64 / b)) }
            }
            (Value::Float(a), Value::Int(b)) => {
                if b == 0 { Err(RuntimeError::new("division by zero")) } else { Ok(Value::Float(a / b as f64)) }
            }
            _ => Err(RuntimeError::new("type error: cannot divide the given operands")),
        }
    }

    fn modulo(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(RuntimeError::new("modulo by zero"))
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if b == 0.0 { 
                    Err(RuntimeError::new("modulo by zero")) 
                } else { 
                    Ok(Value::Float(a % b)) 
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if b == 0.0 { 
                    Err(RuntimeError::new("modulo by zero")) 
                } else { 
                    Ok(Value::Float((a as f64) % b)) 
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if b == 0 { 
                    Err(RuntimeError::new("modulo by zero")) 
                } else { 
                    Ok(Value::Float(a % (b as f64))) 
                }
            }
            _ => Err(RuntimeError::new("type error: cannot apply modulo to the given operands")),
        }
    }

    fn eq(&self, left: Value, right: Value) -> bool {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }

    fn cmp<F>(&self, f: F, left: Value, right: Value) -> Result<Value, RuntimeError>
    where
        F: Fn(f64, f64) -> bool,
    {
        let (a, b) = match (left, right) {
            (Value::Int(a), Value::Int(b)) => (a as f64, b as f64),
            (Value::Float(a), Value::Float(b)) => (a, b),
            (Value::Int(a), Value::Float(b)) => (a as f64, b),
            (Value::Float(a), Value::Int(b)) => (a, b as f64),
            _ => return Err(RuntimeError::new("type error: cannot compare given operands")),
        };
        Ok(Value::Bool(f(a, b)))
    }

    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.env.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_function_sees_global() {
        let src = r#"
            var g = 10;
            func f(): {
                return g;
            } end
            var r = f();
        "#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        assert_eq!(interp.get_global("r"), Some(Value::Int(10)));
    }

    #[test]
    fn test_block_scoping_shadowing() {
        let src = r#"
            var x = 1;
            {
                var x = 2;
            }
        "#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        assert_eq!(interp.get_global("x"), Some(Value::Int(1)));
    }

    #[test]
    fn test_string_escapes() {
        let src = r#"var s = "line1\nline2";"#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        assert_eq!(interp.get_global("s"), Some(Value::String("line1\nline2".to_string())));
    }

    #[test]
    fn test_division_by_zero_error() {
        let src = r#"var x = 1 / 0;"#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        let res = interp.eval_program(&program);
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert!(err.message.contains("division by zero"));
    }

    #[test]
    fn test_table_literal() {
        let src = r#"var t = {x: 10, y: 20};"#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        if let Some(Value::Table(map)) = interp.get_global("t") {
            assert_eq!(map.get("x"), Some(&Value::Int(10)));
            assert_eq!(map.get("y"), Some(&Value::Int(20)));
        } else {
            panic!("expected table");
        }
    }

    #[test]
    fn test_member_access() {
        let src = r#"
            var t = {x: 5};
            var result = t.x;
        "#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        assert_eq!(interp.get_global("result"), Some(Value::Int(5)));
    }

    #[test]
    fn test_index_access() {
        let src = r#"
            var t = {name: "test"};
            var result = t["name"];
        "#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        assert_eq!(interp.get_global("result"), Some(Value::String("test".to_string())));
    }

    #[test]
    fn test_object_declaration() {
        let src = r#"
            object Player: {
                var hp = 100;
            } end
            var p_hp = Player.hp;
        "#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        assert_eq!(interp.get_global("p_hp"), Some(Value::Int(100)));
    }

    #[test]
    fn test_closure_captures_outer_variable() {
        let src = r#"
            var x = 10;
            func makeAdder(): {
                func inner(n): {
                    return x + n;
                } end
                return inner;
            } end
            var adder = makeAdder();
            var result = adder(5);
        "#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().expect("parse failed");
        let mut interp = Interpreter::new();
        interp.eval_program(&program).expect("runtime error");
        assert_eq!(interp.get_global("result"), Some(Value::Int(15)));
    }
}
