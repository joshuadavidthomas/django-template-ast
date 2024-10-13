use django_template_ast::compile;

#[test]
#[should_panic]
fn test_empty_template() {
    let result = compile("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
#[should_panic]
fn test_simple_template() {
    let result = compile("Hello, {{ name }}!");
    assert!(result.is_ok());
    // You'll need to adjust this expected output based on your actual implementation
    assert_eq!(result.unwrap(), "Hello, {{ name }}!");
}

#[test]
#[should_panic]
fn test_invalid_template() {
    let result = compile("{% invalid %}");
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_complex_template() {
    let template = r#"
        {% if user.is_authenticated %}
            Hello, {{ user.name }}!
        {% else %}
            Please log in.
        {% endif %}
    "#;
    let result = compile(template);
    assert!(result.is_ok());

    if let Ok(compiled) = result {
        assert!(compiled.contains("Hello") && compiled.contains("Please log in"));
    } else {
        panic!("Compilation failed unexpectedly");
    }
}
