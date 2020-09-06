use wasm_bindgen::prelude::*;
use minidecaf::{parser::*, ir::*, codegen::*};
use std::fmt::Write;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn set_err(msg: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
  std::panic::set_hook(Box::new(|info| {
    let payload = info.payload();
    let msg = payload.downcast_ref::<String>().map(|x| x.as_str())
      .or_else(|| payload.downcast_ref::<&str>().copied()).unwrap_or("unknown error");
    set_err(msg);
  }));
}

#[wasm_bindgen]
pub fn work(act: &str, input: &str) -> String {
  let mut lexer = Lexer::new(input.as_bytes());
  if act == "lex" {
    let mut ret = String::new();
    loop {
      let token = lexer.next();
      let _ = writeln!(ret, "{:?}", token);
      if token.kind == TokenKind::_Eof || token.kind == TokenKind::_Err { break; }
    }
    return ret;
  }
  let ast = match (Parser {}).parse(&mut lexer) { Ok(x) => x, Err(t) => return format!("failed to parse at token {:?}", t) };
  if act == "ast" { return format!("{:#?}", ast); }
  // ast2ir(&ast);
  let ir = ast2ir(&ast);
  if act == "ir" {
    let mut ret = String::new();
    for f in &ir.funcs {
      let _ = writeln!(ret, "{}(param_cnt = {}, var_cnt = {}):", f.name, f.param_cnt, f.var_cnt);
      for s in &f.stmts { let _ = writeln!(ret, "  {:?}", s); }
      ret.push('\n');
    }
    return ret;
  }
  let mut ret = Vec::new();
  let _ = write_asm(&ir, &mut ret);
  unsafe { String::from_utf8_unchecked(ret) }
}