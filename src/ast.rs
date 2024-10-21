use crate::error::{AstError, NodeError};
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    HTMLElement {
        tag: String,
        attributes: Vec<(String, AttributeValue)>,
        children: Vec<Node>,
    },
    HTMLVoidElement {
        tag: String,
        attributes: Vec<(String, AttributeValue)>,
    },
    HTMLComment(String),
    HTMLDoctype {
        doctype: String,
    },
    Script {
        attributes: Vec<(String, AttributeValue)>,
        content: String,
    },
    Style {
        attributes: Vec<(String, AttributeValue)>,
        content: String,
    },
    DjangoVariable(String),
    DjangoBlock {
        name: String,
        arguments: Vec<String>,
        children: Vec<Node>,
    },
    DjangoComment(String),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub enum AttributeValue {
    Value(String),
    Boolean,
}

impl Node {
    pub fn new_html_element(
        tag: String,
        attributes: Option<Vec<(String, AttributeValue)>>,
        children: Option<Vec<Node>>,
    ) -> Result<Self, NodeError> {
        if tag.is_empty() {
            return Err(NodeError::NoTagName);
        };

        let attributes = attributes.unwrap_or_default();
        let children = children.unwrap_or_default();

        Ok(Node::HTMLElement {
            tag,
            attributes,
            children,
        })
    }

    pub fn new_html_void_element(
        tag: String,
        attributes: Option<Vec<(String, AttributeValue)>>,
    ) -> Result<Self, NodeError> {
        if tag.is_empty() {
            return Err(NodeError::NoTagName);
        };

        let attributes = attributes.unwrap_or_default();

        Ok(Node::HTMLVoidElement { tag, attributes })
    }

    pub fn new_html_comment(content: String) -> Result<Self, NodeError> {
        Ok(Node::HTMLComment(content))
    }

    pub fn new_html_doctype(doctype: String) -> Result<Self, NodeError> {
        Ok(Node::HTMLDoctype { doctype })
    }

    pub fn new_script(
        attributes: Option<Vec<(String, AttributeValue)>>,
        content: String,
    ) -> Result<Self, NodeError> {
        let attributes = attributes.unwrap_or_default();

        Ok(Node::Script {
            attributes,
            content,
        })
    }

    pub fn new_style(
        attributes: Option<Vec<(String, AttributeValue)>>,
        content: String,
    ) -> Result<Self, NodeError> {
        let attributes = attributes.unwrap_or_default();

        Ok(Node::Style {
            attributes,
            content,
        })
    }

    pub fn new_django_variable(content: String) -> Result<Self, NodeError> {
        Ok(Node::DjangoVariable(content))
    }

    pub fn new_django_block(
        name: String,
        arguments: Option<Vec<String>>,
        children: Option<Vec<Node>>,
    ) -> Result<Self, NodeError> {
        if name.is_empty() {
            return Err(NodeError::NoBlockName);
        };

        let arguments = arguments.unwrap_or_default();
        let children = children.unwrap_or_default();

        Ok(Node::DjangoBlock {
            name,
            arguments,
            children,
        })
    }

    pub fn new_django_comment(content: String) -> Result<Self, NodeError> {
        Ok(Node::DjangoComment(content))
    }

    pub fn new_text(content: String) -> Result<Self, NodeError> {
        Ok(Node::Text(content))
    }
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub nodes: Vec<Node>,
}

impl Ast {
    pub fn new() -> Self {
        Ast { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_html_element() {
        let node = Node::new_html_element("html".to_string(), None, None).unwrap();

        if let Node::HTMLElement {
            tag,
            attributes,
            children,
        } = node
        {
            assert_eq!(tag, "html");
            assert!(attributes.is_empty());
            assert!(children.is_empty());
        } else {
            panic!("Expected an HTMLElement node");
        }
    }

    #[test]
    fn test_new_html_element_with_attributes_and_children() {
        let attributes = vec![
            (
                "class".to_string(),
                AttributeValue::Value("container".to_string()),
            ),
            ("disabled".to_string(), AttributeValue::Boolean),
        ];
        let children = vec![
            Node::new_html_element("div".to_string(), None, None).unwrap(),
            Node::new_html_element("span".to_string(), None, None).unwrap(),
        ];

        let node =
            Node::new_html_element("div".to_string(), Some(attributes), Some(children)).unwrap();

        if let Node::HTMLElement {
            tag,
            attributes,
            children,
        } = node
        {
            assert_eq!(tag, "div");

            assert_eq!(attributes.len(), 2);
            assert_eq!(attributes[0].0, "class");
            assert_eq!(attributes[1].0, "disabled");
            assert!(
                matches!(&attributes[0].1, AttributeValue::Value(value) if value == "container")
            );
            assert!(matches!(&attributes[1].1, AttributeValue::Boolean));

            assert_eq!(children.len(), 2);
            assert!(matches!(&children[0], Node::HTMLElement { tag, .. } if tag == "div"));
            assert!(matches!(&children[1], Node::HTMLElement { tag, .. } if tag == "span"));
        } else {
            panic!("Expected an HTMLElement node");
        }
    }

    #[test]
    fn test_new_html_element_empty_tag() {
        let result = Node::new_html_element("".to_string(), None, None);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NodeError::NoTagName));
    }

    #[test]
    fn test_new_html_void_element() {
        let node = Node::new_html_void_element("html".to_string(), None).unwrap();

        if let Node::HTMLVoidElement { tag, attributes } = node {
            assert_eq!(tag, "html");
            assert!(attributes.is_empty());
        } else {
            panic!("Expected an HTMLVoidElement node");
        }
    }

    #[test]
    fn test_new_html_comment() {
        let node = Node::new_html_comment("A comment".to_string()).unwrap();

        if let Node::HTMLComment(content) = node {
            assert_eq!(content, "A comment");
        } else {
            panic!("Expected an HTMLComment node");
        }
    }

    #[test]
    fn test_new_html_doctype() {
        let node = Node::new_html_doctype("html".to_string()).unwrap();

        if let Node::HTMLDoctype { doctype } = node {
            assert_eq!(doctype, "html");
        } else {
            panic!("Expected an HTMLDoctype node");
        }
    }

    #[test]
    fn test_new_script() {
        let node = Node::new_script(None, "console.log('hello');".to_string()).unwrap();

        if let Node::Script {
            attributes,
            content,
        } = node
        {
            assert!(attributes.is_empty());
            assert_eq!(content, "console.log('hello');");
        } else {
            panic!("Expected a Script node");
        }
    }

    #[test]
    fn test_new_script_with_attributes() {
        let attributes = vec![(
            "src".to_string(),
            AttributeValue::Value("javascript.js".to_string()),
        )];
        let node = Node::new_script(Some(attributes), "console.log('hello');".to_string()).unwrap();

        if let Node::Script {
            attributes,
            content,
        } = node
        {
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].0, "src");
            assert!(
                matches!(&attributes[0].1, AttributeValue::Value(value) if value == "javascript.js")
            );

            assert_eq!(content, "console.log('hello');");
        } else {
            panic!("Expected a Script node");
        }
    }

    #[test]
    fn test_new_style() {
        let node = Node::new_style(None, "body { background-color: red; }".to_string()).unwrap();

        if let Node::Style {
            attributes,
            content,
        } = node
        {
            assert!(attributes.is_empty());
            assert_eq!(content, "body { background-color: red; }");
        } else {
            panic!("Expected a Style node");
        }
    }

    #[test]
    fn test_new_style_with_attributes() {
        let attributes = vec![(
            "media".to_string(),
            AttributeValue::Value("max-width: 500px".to_string()),
        )];
        let node = Node::new_style(
            Some(attributes),
            "body { background-color: red; }".to_string(),
        )
        .unwrap();

        if let Node::Style {
            attributes,
            content,
        } = node
        {
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].0, "media");
            assert!(
                matches!(&attributes[0].1, AttributeValue::Value(value) if value == "max-width: 500px")
            );

            assert_eq!(content, "body { background-color: red; }");
        } else {
            panic!("Expected a Style node");
        }
    }

    #[test]
    fn test_new_django_variable() {
        let node = Node::new_django_variable("variable".to_string()).unwrap();

        if let Node::DjangoVariable(content) = node {
            assert_eq!(content, "variable");
        } else {
            panic!("Expected a DjangoVariable node");
        }
    }

    #[test]
    fn test_new_django_block() {
        let node = Node::new_django_block("dj_block".to_string(), None, None).unwrap();

        if let Node::DjangoBlock {
            name,
            arguments,
            children,
        } = node
        {
            assert_eq!(name, "dj_block");
            assert!(arguments.is_empty());
            assert!(children.is_empty());
        } else {
            panic!("Expected a DjangoBlock node");
        }
    }

    #[test]
    fn test_new_django_block_with_arguments_and_children() {
        let arguments = vec![
            "arg1".to_string(),
            "arg2=variable".to_string(),
            "arg3='string'".to_string(),
        ];
        let children = vec![Node::new_html_element("div".to_string(), None, None).unwrap()];

        let node = Node::new_django_block("dj_block".to_string(), Some(arguments), Some(children))
            .unwrap();

        if let Node::DjangoBlock {
            name,
            arguments,
            children,
        } = node
        {
            assert_eq!(name, "dj_block");

            assert_eq!(arguments.len(), 3);
            assert_eq!(arguments[0], "arg1");
            assert_eq!(arguments[1], "arg2=variable");
            assert_eq!(arguments[2], "arg3='string'");

            assert_eq!(children.len(), 1);
            assert!(matches!(&children[0], Node::HTMLElement { tag, .. } if tag == "div"));
        } else {
            panic!("Expected a DjangoBlock node");
        }
    }

    #[test]
    fn test_new_django_block_empty_name() {
        let result = Node::new_django_block("".to_string(), None, None);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NodeError::NoBlockName));
    }

    #[test]
    fn test_new_django_comment() {
        let node = Node::new_django_comment("A comment".to_string()).unwrap();

        if let Node::DjangoComment(content) = node {
            assert_eq!(content, "A comment");
        } else {
            panic!("Expected a DjangoComment node");
        }
    }

    #[test]
    fn test_new_text() {
        let node = Node::new_text("Some text".to_string()).unwrap();

        if let Node::Text(content) = node {
            assert_eq!(content, "Some text");
        } else {
            panic!("Expected a Text node");
        }
    }
}
