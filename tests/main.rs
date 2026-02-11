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
