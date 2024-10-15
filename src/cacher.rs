pub struct Cacher<T, E>
  where T: Fn(E) -> E,
{
  query: T,
  value: Option<E>,
}

impl<T,E> Cacher<T,E>
  where T: Fn(E) -> E, E: Clone
{
  fn new(query: T) -> Cacher<T,E> {
    Cacher {
        query,
        value: None,
    }
  }

  // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
  fn value(&mut self, arg: E) -> E {
    match &self.value {
        Some(v) => v.clone(),
        None => {
            let v = (self.query)(arg);
            self.value = Some(v.clone());
            v.clone()
        }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cacher() {
    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(0), 1);
    assert_eq!(cacher.value(0), 1);
    assert_eq!(cacher.value(1), 1);
  }

  #[test]
  fn test_cacher_string() {
    let mut cacher = Cacher::new(|x: String| String::from("Hello ") + &x);
    assert_eq!(cacher.value(String::from("world")), "Hello world");
    assert_eq!(cacher.value(String::from("world second time")), "Hello world");
  }
}