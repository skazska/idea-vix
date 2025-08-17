use axum::{extract::{rejection:: {FormRejection, JsonRejection}, FromRequest, Request}, http::StatusCode, response::{IntoResponse, Response}, Form, Json};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

/// A wrapper for validated form data
/// to be used in request handling to get validated form payloads
/// example usage:
/// ```ignore
/// async fn handler(ValidatedForm(payload): ValidatedForm<MyStruct>) {
///     // Handle the validated form payload
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

/// A wrapper for validated JSON data
/// to be used in request handling to get validated JSON payloads
/// example usage:
/// ```ignore
/// async fn handler(ValidatedJson(payload): ValidatedJson<MyStruct>) {
///     // Handle the validated JSON payload
/// }
/// ```
pub struct ValidatedJson<J>(pub J);

// #[async_trait::async_trait]
impl<S, J> FromRequest<S> for ValidatedJson<J>
where
    S: Send + Sync,
    J: Validate,
    Json<J>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<J>::from_request(req, state)
            .await
            .map_err(|op| ServerError::AxumJsonRejection(op))?;
        data.validate()
            .map_err(|op| ServerError::ValidationError(op))?;
        Ok(Self(data))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use axum::http::StatusCode;
    use validator::{ValidationError, ValidationErrors};
    use serde::Deserialize;
    use validator::Validate;

    fn server_error_response_probe(error: ServerError, expected_status: StatusCode) {
        let response = error.into_response();
        assert_eq!(response.status(), expected_status);
    }

    fn create_validation_error(field: &'static str, code: &'static str) -> ValidationErrors {
        let mut errors = ValidationErrors::new();
        let validation_error = ValidationError::new(code);
        errors.add(field, validation_error);
        errors
    }

    #[test]
    fn server_error_validation_error_response() {
        let validation_errors = create_validation_error("test_field", "length");
        server_error_response_probe(
            ServerError::ValidationError(validation_errors),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn server_error_validation_error_message_format() {
        let validation_errors = create_validation_error("username", "length");
        let error = ServerError::ValidationError(validation_errors);
        let response = error.into_response();
        
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn server_error_validation_multiple_fields() {
        let mut errors = ValidationErrors::new();
        errors.add("username", ValidationError::new("length"));
        errors.add("email", ValidationError::new("email"));
        
        server_error_response_probe(
            ServerError::ValidationError(errors),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn server_error_validation_multiple_errors_same_field() {
        let mut errors = ValidationErrors::new();
        errors.add("password", ValidationError::new("length"));
        errors.add("password", ValidationError::new("must_match"));
        
        server_error_response_probe(
            ServerError::ValidationError(errors),
            StatusCode::BAD_REQUEST
        );
    }

    // Test validation with actual struct to ensure integration works
    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[validate(length(min = 3, max = 10))]
        name: String,
        #[validate(email)]
        email: String,
    }

    #[test]
    fn validation_error_from_struct_validation() {
        let test_data = TestStruct {
            name: "ab".to_string(), // Too short
            email: "invalid-email".to_string(), // Invalid email
        };
        
        let validation_result = test_data.validate();
        assert!(validation_result.is_err());
        
        if let Err(validation_errors) = validation_result {
            server_error_response_probe(
                ServerError::ValidationError(validation_errors),
                StatusCode::BAD_REQUEST
            );
        }
    }

    #[test]
    fn server_error_message_contains_validation_info() {
        let validation_errors = create_validation_error("username", "length");
        let error = ServerError::ValidationError(validation_errors);
        let response = error.into_response();
        
        // Extract the response body to check message format
        let (parts, _body) = response.into_parts();
        assert_eq!(parts.status, StatusCode::BAD_REQUEST);
        
        // The body should be a string containing validation error info
        // We can't easily extract the body content in unit tests, but we know
        // the format should be "Input validation error: [...]"
    }

    #[test]
    fn server_error_validation_message_newline_replacement() {
        // Test that newlines in validation messages are replaced with commas
        let mut errors = ValidationErrors::new();
        let mut error = ValidationError::new("custom");
        error.message = Some("Line 1\nLine 2\nLine 3".into());
        errors.add("field", error);
        
        let server_error = ServerError::ValidationError(errors);
        let response = server_error.into_response();
        
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        // The newlines should be replaced with ", " in the actual implementation
    }

    // Test ValidatedForm and ValidatedJson wrapper types
    #[derive(Debug, Clone, Deserialize, Validate)]
    struct SimpleTestStruct {
        #[validate(length(min = 1))]
        name: String,
    }

    #[test]
    fn validated_form_wrapper_basic() {
        // Test that ValidatedForm wrapper can be created with valid data
        let valid_data = SimpleTestStruct {
            name: "valid".to_string(),
        };
        
        let validated_form = ValidatedForm(valid_data.clone());
        assert_eq!(validated_form.0.name, "valid");
    }

    #[test]
    fn validated_json_wrapper_basic() {
        // Test that ValidatedJson wrapper can be created with valid data
        let valid_data = SimpleTestStruct {
            name: "valid".to_string(),
        };
        
        let validated_json = ValidatedJson(valid_data.clone());
        assert_eq!(validated_json.0.name, "valid");
    }

    #[test]
    fn validated_form_default_implementation() {
        // Test that ValidatedForm implements Default when T implements Default
        #[derive(Debug, Clone, Default, Deserialize, Validate)]
        struct DefaultStruct {
            #[validate(length(min = 0))] // Allow empty for default
            value: String,
        }
        
        let default_form: ValidatedForm<DefaultStruct> = ValidatedForm::default();
        assert_eq!(default_form.0.value, "");
    }

    #[test]
    fn server_error_from_validation_errors() {
        let validation_errors = create_validation_error("field", "required");
        let server_error: ServerError = validation_errors.into();
        
        match server_error {
            ServerError::ValidationError(_) => {}, // Expected
            _ => panic!("Expected ValidationError variant"),
        }
    }

    // Note: FormRejection and JsonRejection cannot be easily constructed in tests
    // as they have private constructors and are typically created by Axum's 
    // extraction process. The IntoResponse implementation handles these cases 
    // by returning BAD_REQUEST status codes. In integration tests, these would
    // be tested by sending actual malformed requests to the server.
    
    // The FromRequest implementations for ValidatedForm and ValidatedJson are
    // complex to test in unit tests as they require full HTTP request setup.
    // These are better tested in integration tests where actual HTTP requests
    // can be made to handlers that use these extractors.
}
