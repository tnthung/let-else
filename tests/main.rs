

#[cfg(test)]
mod tests {
  use let_else::let_else;


  #[test]
  fn test1() {
    fn foo(value: Result<i32, String>) -> String {
      let_else!(Ok(value) = value else {
        return format!("Error");
      });

      return format!("Value: {}", value);
    }

    assert_eq!(foo(Ok(42)), "Value: 42");
    assert_eq!(foo(Err("Something went wrong".to_string())), "Error");
  }


  #[test]
  fn test2() {
    fn foo(value: Result<i32, String>) -> String {
      let_else!(Ok(value) = value else as Err(err) {
        return format!("Error: {}", err);
      });

      return format!("Value: {}", value);
    }

    assert_eq!(foo(Ok(42)), "Value: 42");
    assert_eq!(foo(Err("Something went wrong".to_string())), "Error: Something went wrong");
  }


  #[test]
  fn test3() {
    fn foo(value: Result<i32, String>) -> String {
      let_else!(Ok(value) = value else as err {
        return format!("Error: {:?}", err);
      });

      return format!("Value: {}", value);
    }

    assert_eq!(foo(Ok(42)), "Value: 42");
    assert_eq!(foo(Err("Something went wrong".to_string())), "Error: Err(\"Something went wrong\")");
  }


  #[test]
  fn test4() {
    #[allow(dead_code)]
    enum Triple {
      A(i32),
      B(String),
      C(f64),
    }

    fn foo(value: Triple) -> String {
      let_else!(Triple::A(value) = value else as Triple::B(_) | Triple::C(_) {
        return format!("Error: B or C");
      });

      return format!("Value: {}", value);
    }

    assert_eq!(foo(Triple::A(42)), "Value: 42");
    assert_eq!(foo(Triple::B("Hello".to_string())), "Error: B or C");
    assert_eq!(foo(Triple::C(3.14)), "Error: B or C");
  }


  #[test]
  fn test5() {
    #[allow(dead_code)]
    enum Triple {
      A(i32),
      B(String),
      C(f64),
    }

    fn foo(value: Triple) -> String {
      let_else!(Triple::A(value) = value else match {
        Triple::B(b) => { return format!("Error: B with value {}", b); },
        Triple::C(c) => { return format!("Error: C with value {}", c); },
      });

      return format!("Value: {}", value);
    }

    assert_eq!(foo(Triple::A(42)), "Value: 42");
    assert_eq!(foo(Triple::B("Hello".to_string())), "Error: B with value Hello");
    assert_eq!(foo(Triple::C(3.14)), "Error: C with value 3.14");
  }


  #[test]
  fn test6() {
    #[allow(dead_code)]
    enum Triple {
      A(i32),
      B(u32),
      C(u32),
    }

    fn foo(value: Triple) -> String {
      let_else!(Triple::A(value) = value else as | Triple::B(y) | Triple::C(y) {
        return format!("Error: B or C with value {}", y);
      });

      return format!("Value: {}", value);
    }

    assert_eq!(foo(Triple::A(42)), "Value: 42");
    assert_eq!(foo(Triple::B(100)), "Error: B or C with value 100");
    assert_eq!(foo(Triple::C(200)), "Error: B or C with value 200");
  }
}
