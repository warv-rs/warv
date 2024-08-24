use std::collections::HashMap;

/// URI Struct
#[derive(Debug, Clone)]
pub struct Uri {
    pub path: String,
    pub query: Option<HashMap<String, String>>,
    pub pattern_params: Option<HashMap<String, String>>,
}
impl Uri {
    pub fn new(path: &str) -> Self {
        let mut query_values = path.split("?");
        let p = query_values.next();
        let qv = match query_values.next() {
            Some(q) => Some(
                q.split('&')
                    .filter_map(|pair| {
                        let mut split = pair.splitn(2, '=');
                        Some((
                            split.next()?.to_string(),
                            split.next().unwrap_or("").to_string(),
                        ))
                    })
                    .collect::<HashMap<String, String>>(),
            ),
            None => None,
        };
        return Uri {
            path: p.unwrap().to_owned(),
            query: qv,
            pattern_params: None,
        };
    }
    /// Returns the path
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
    /// Returns the query parameters
    pub fn query(&self, id: &str) -> Option<String> {
        if self.query.is_some() {
            return self.query.as_ref()?.get(id).cloned();
        }
        return None;
    }
    /// Returns the path parameters
    pub fn param(&self, id: &str) -> Option<String> {
        if self.pattern_params.is_some() {
            return self.pattern_params.as_ref()?.get(id).cloned();
        }
        return None;
    }
}
