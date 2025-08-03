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

