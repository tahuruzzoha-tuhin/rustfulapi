pub enum ResultControlFlow<T, E> {
  Ok(T),
  Err(E),
  Break,
  Continue,
}

impl<T, E> ResultControlFlow<T, E> {
  pub fn is_break(&self) -> bool {
    matches!(self, Self::Break)
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, Self::Ok(_))
  }
  pub fn is_err(&self) -> bool {
    matches!(self, Self::Err(_))
  }
  pub fn is_continue(&self) -> bool {
    matches!(self, Self::Continue)
  }
}
