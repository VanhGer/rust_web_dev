use handle_errors::CustomError;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Default, Debug, PartialEq)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    pub limit: Option<u32>, 
    /// The index of the first item which has to be returned
    pub offset: u32, 
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, Some(1));
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, CustomError> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query
            // and tries to convert it to a number
            limit: Some(params
                .get("limit")
                .unwrap()
                .parse::<u32>()
                .map_err(CustomError::ParseError)?),

            // Takes the "offset" parameter in the query
            // and tries to convert it to a number
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(CustomError::ParseError)?,
        });
    }
    Err(CustomError::MissingParameters)
}


#[cfg(test)] 
mod pagination_tests { 
    use super::{HashMap, extract_pagination, Pagination, CustomError}; 

    #[test] 
    fn valid_pagination() { 
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("1"));
        let pagination_result = extract_pagination(params);
        let expected = Pagination { 
            limit: Some(1),
            offset: 1
        };
        assert_eq!(pagination_result.unwrap(), expected); 
    }
}

#[test]
fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = format!("{}", CustomError::MissingParameters);

        assert_eq!(pagination_result, expected);
}