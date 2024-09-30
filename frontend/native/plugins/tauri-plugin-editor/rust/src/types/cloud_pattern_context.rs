use serde_json::{Map, Value as JsonValue};

use crate::{error::InternalApplicationError, EditorError};

#[derive(Default)]
pub struct CloudPatternContext {
    current_namespace: Vec<String>,
    context: Map<String, JsonValue>,
}

impl CloudPatternContext {
    pub(crate) fn clear(&mut self) {
        self.context.clear();
        self.current_namespace.clear();
    }

    pub(crate) fn push_to_context(
        &mut self,
        key: String,
        value: JsonValue,
    ) -> Result<(), EditorError> {
        let mut current_ctx = &mut self.context;

        for ns in self.current_namespace.iter() {
            let ctx = current_ctx
                .get_mut(ns)
                .ok_or_else(|| InternalApplicationError::NamespaceNotInContext {
                    namespace: ns.to_owned(),
                })?
                .as_object_mut()
                .ok_or_else(|| InternalApplicationError::NamespaceNotAnObject {
                    namespace: ns.to_owned(),
                })?;
            current_ctx = ctx;
        }

        current_ctx.insert(key, value);

        Ok(())
    }

    pub(crate) fn get_current_context(&self) -> Result<&Map<String, JsonValue>, EditorError> {
        let mut current_ctx = &self.context;

        for ns in self.current_namespace.iter() {
            let ctx = current_ctx
                .get(ns)
                .ok_or_else(|| InternalApplicationError::NamespaceNotInContext {
                    namespace: ns.to_owned(),
                })?
                .as_object()
                .ok_or_else(|| InternalApplicationError::NamespaceNotAnObject {
                    namespace: ns.to_owned(),
                })?;
            current_ctx = ctx;
        }

        Ok(current_ctx)
    }

    pub(crate) fn push_context_namespace(&mut self, namespace: String) -> Result<(), EditorError> {
        if !self.get_current_context()?.contains_key(&namespace) {
            self.push_to_context(namespace.to_owned(), JsonValue::Object(Map::new()))?;
        }

        self.current_namespace.push(namespace);

        Ok(())
    }

    pub(crate) fn pop_context_namespace(&mut self) -> Result<(), EditorError> {
        self.current_namespace.pop();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_clear() {
        let mut ctx = CloudPatternContext::default();
        ctx.context.insert("key".to_string(), json!("value"));
        ctx.current_namespace.push("namespace".to_string());

        ctx.clear();

        assert!(ctx.context.is_empty());
        assert!(ctx.current_namespace.is_empty());
    }

    #[test]
    fn test_push_to_context() {
        let mut ctx = CloudPatternContext::default();

        // Test pushing to root context
        ctx.push_to_context("key1".to_string(), json!("value1"))
            .unwrap();
        assert_eq!(ctx.context.get("key1").unwrap(), &json!("value1"));

        // Test pushing to nested context
        ctx.push_context_namespace("ns1".to_string()).unwrap();
        ctx.push_to_context("key2".to_string(), json!("value2"))
            .unwrap();

        let ns1_ctx = ctx.context.get("ns1").unwrap().as_object().unwrap();
        assert_eq!(ns1_ctx.get("key2").unwrap(), &json!("value2"));
    }

    #[test]
    fn test_get_current_context() {
        let mut ctx = CloudPatternContext::default();
        ctx.push_to_context("key1".to_string(), json!("value1"))
            .unwrap();

        // Test getting root context
        let root_ctx = ctx.get_current_context().unwrap();
        assert_eq!(root_ctx.get("key1").unwrap(), &json!("value1"));

        // Test getting nested context
        ctx.push_context_namespace("ns1".to_string()).unwrap();
        ctx.push_to_context("key2".to_string(), json!("value2"))
            .unwrap();

        let ns1_ctx = ctx.get_current_context().unwrap();
        assert_eq!(ns1_ctx.get("key2").unwrap(), &json!("value2"));
    }

    #[test]
    fn test_push_context_namespace() {
        let mut ctx = CloudPatternContext::default();

        ctx.push_context_namespace("ns1".to_string()).unwrap();
        assert_eq!(ctx.current_namespace, vec!["ns1".to_string()]);

        ctx.push_context_namespace("ns2".to_string()).unwrap();
        assert_eq!(
            ctx.current_namespace,
            vec!["ns1".to_string(), "ns2".to_string()]
        );

        // Test pushing existing namespace
        ctx.push_context_namespace("ns1".to_string()).unwrap();
        assert_eq!(
            ctx.current_namespace,
            vec!["ns1".to_string(), "ns2".to_string(), "ns1".to_string()]
        );
    }

    #[test]
    fn test_pop_context_namespace() {
        let mut ctx = CloudPatternContext::default();

        ctx.push_context_namespace("ns1".to_string()).unwrap();
        ctx.push_context_namespace("ns2".to_string()).unwrap();

        ctx.pop_context_namespace().unwrap();
        assert_eq!(ctx.current_namespace, vec!["ns1".to_string()]);

        ctx.pop_context_namespace().unwrap();
        assert!(ctx.current_namespace.is_empty());

        // Test popping from empty namespace
        ctx.pop_context_namespace().unwrap();
        assert!(ctx.current_namespace.is_empty());
    }

    #[test]
    fn test_error_cases() {
        let mut ctx = CloudPatternContext::default();

        // Test pushing to non-existent namespace
        ctx.current_namespace.push("non_existent".to_string());
        let result = ctx.push_to_context("key".to_string(), json!("value"));
        let error_message = result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            EditorError::InternalApplicationError(
                InternalApplicationError::NamespaceNotInContext {
                    namespace: "non_existent".to_owned()
                }
                .to_string()
            )
            .to_string()
        );

        // Test pushing to non-object namespace
        ctx.clear();
        ctx.push_to_context("not_object".to_string(), json!("string"))
            .unwrap();
        ctx.current_namespace.push("not_object".to_string());
        let result = ctx.push_to_context("key".to_string(), json!("value"));
        let error_message = result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            EditorError::InternalApplicationError(
                InternalApplicationError::NamespaceNotAnObject {
                    namespace: "not_object".to_owned()
                }
                .to_string()
            )
            .to_string()
        );

        // Test getting non-existent namespace
        ctx.clear();
        ctx.current_namespace.push("non_existent".to_string());
        let result = ctx.get_current_context();
        let error_message = result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            EditorError::InternalApplicationError(
                InternalApplicationError::NamespaceNotInContext {
                    namespace: "non_existent".to_owned()
                }
                .to_string()
            )
            .to_string()
        );
    }
}
