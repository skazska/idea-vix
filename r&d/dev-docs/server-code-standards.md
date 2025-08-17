# Code standards

## Modularity and Extensibility

Code should be modular and extensible, allowing for easy addition of new features or modifications to existing ones without significant refactoring.

Logical separation of fieatures and functionality is achieved through units and modules.

Units are usually devoted to specific functionality or features, such as package management, boards, or user sessions. Each module should encapsulate its own logic and data structures.

Internal unit structure consists of layers.

Layers may include modules such as `api`, `service`, and `storage` and use common modules like `error` and `db`.

### Typical layer structure

- **api**: Contains the API endpoints and request handlers. Converts payload/params to model and dispatch call to service layer. converts service results to API response.
- **service**: Service layer implements business logic, which may include:
  - Validating input data
  - Performing calculations or transformations
  - Interacting with the storage layer to retrieve or modify data
  - Interacting with other services
  - Interacting with the external APIs
  - Returning results or errors
- **storage**: Contains the database models and data access logic.

### Typical execution flow

1. API layer receives and validates request. Converts payload/params to model and dispatch call to service layer.
2. Service layer implements business logic including access to storage layer.
3. Storage layer interacts with the database to retrieve or modify data.
4. Service layer returns the result to the API layer.
5. API layer sends the final response to the client.

### Dependency sanity

There should be no circular dependencies between modules. Each module should depend only on the modules that are necessary for its functionality.
There could be common units of modules that are used by multiple modules, such as `error`, `db`, or `config`. These common modules should not depend on any specific feature modules, and better not depend on other common units of modules if possible.
Feature modules should not depend on each other directly. Instead, they may interact through the API layer or service layer, which acts as a mediator, or there should be integral units of modules that provide functionality involving multiple feature modules.

## Unit Testing Standards

Unit tests should provide comprehensive coverage for individual functions and methods while maintaining readability and ease of maintenance.

### Test Organization

- Tests should be placed in a `#[cfg(test)]` module within the same file as the code being tested
- Use descriptive test function names that clearly indicate what is being tested
- Group related tests with clear section comments (e.g., `// --- AuthToken tests ---`)

### Helper Functions

Use helper functions to reduce boilerplate and improve test readability:

```rust
// Helper for testing conversions with expected results
fn conversion_probe(input: InputType, expected: OutputType) {
    let result = input.convert();
    assert_eq!(result, expected);
}

// Helper for testing responses with expected status codes
fn response_probe(error: ErrorType, expected_status: StatusCode) {
    let response = error.into_response();
    assert_eq!(response.status(), expected_status);
}
```

### Test Coverage Principles

1. **Pure functions**: Test all branches and edge cases
2. **Conversion methods**: Test all variants/cases with expected outputs
3. **Error handling**: Test all error types map to correct responses
4. **Integration points**: Test actual usage patterns where possible

### Test Structure

Each test should follow a clear structure:

```rust
#[test]
fn descriptive_test_name() {
    // Arrange - set up test data
    let input = create_test_data();
    
    // Act & Assert - use helper functions when possible
    helper_probe(input, expected_result);
}
```

### Async Testing

For async functions, use `#[tokio::test]`:

```rust
#[tokio::test]
async fn async_function_test() {
    let result = async_function().await;
    assert_eq!(result.unwrap(), expected_value);
}
```

### Testing Guidelines

- **Focus on behavior**: Test what the function does, not how it does it
- **Use realistic data**: Test with data that resembles actual usage
- **Test edge cases**: Include boundary conditions and error scenarios
- **Keep tests simple**: Each test should verify one specific behavior
- **Avoid mocking when possible**: Use real data structures for unit tests
- **Regression protection**: Tests should catch breaking changes in behavior

### Examples

Good test naming:

- `deserialize_some_missing_field()`
- `auth_token_from_authorization_header()`
- `model_error_forbidden_conversion()`

Good helper usage:

- `deserialize_some_probe(json, expected_struct)`
- `auth_token_probe(headers, expected_token)`
- `model_error_to_status_probe(error, expected_status, expected_message)`

### Test Maintenance

- Update tests when changing function behavior
- Remove obsolete tests when removing functionality  
- Keep test helpers aligned with production code patterns
- Ensure tests remain fast and deterministic

