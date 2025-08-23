---
description: Frontend (SolidJS/TypeScript) specific instructions
---
applyTo: "ui/web/**"
---

# Frontend Specific Instructions

## Stack
- SolidJS
- TypeScript
- Tailwind CSS

## Code Standards
- Leverage TypeScript for type safety
- Define types/interfaces for API responses
- Typed components

## Testing

### E2E with Playwright
- Location: `tests-e2e/` directory
- Naming: `*_flow.spec.ts` for user journey tests
- Requires: Backend server running
- Focus: Critical user flows, not unit-level testing
- Use test helpers for common actions (e.g., login) in `tests-e2e/helpers/`

## Development Workflow
- Always use: `ui/web/` for all npm commands
- Dev server: `npm run dev` (http://localhost:5173)
- Build: `npm run build`
- E2E tests: `npm run test:e2e` (requires backend running)
- Headed mode: `npm run test:e2e:headed`

## Code Organization Patterns (Layered Architecture)

### Component Structure Pattern
- Component Classes: root, feature, utility, service.
- Component Types: visual, provider, API-service, generator.
- Component compositions.

### Routing Conventions
- File-based routing: with lazy-loaded components
- Route constants in `feature/const.ts` files
- Preload functions for data fetching before route activation
- Type-safe params using SolidJS router types

### State Management (Provider Pattern)
- Use API documentation
- Backend provider: Centralized API client in `common/providers/backend.tsx`
- Feature providers: API functions in `feature/providers/api.ts`, data-providers in `feature/providers/provider.tsx`
- Auth state: Session management through backend provider
- No global state library - use SolidJS context and providers

### Form Handling
- Use: `@modular-forms/solid` for form state
- Validation: `valibot` for schema validation
- Pattern: Define schema, create form, bind to inputs
- Type safety: Infer types from valibot schemas

### Component Conventions
- Composition over inheritance
- Props interfaces with clear naming
- Responsive design with Tailwind CSS classes
- Lazy loading for route components
- Small, focused components over large ones

### Styling (Tailwind CSS)
- Utility-first approach with Tailwind classes
- Component composition for reusable patterns
- Responsive design with built-in breakpoints
- Custom utilities in index.css if needed


## Environment & Configuration

### Development Setup
- **Vite config**: `vite.config.ts` for build settings
- **TypeScript**: `tsconfig.json` for compiler options
- **Environment**: Use Vite's built-in env variable handling

### Environment Variables
- **Prefix**: `VITE_` for client-side variables
- **Backend URL**: Typically proxied through Vite dev server
- **API endpoints**: Configure in backend provider

## Code Standards

### TypeScript
- **Strict mode** enabled in tsconfig
- **Interface over type** for object shapes
- **Explicit return types** for functions
- **Const assertions** for literal types

### Import/Export
- **Default exports** for components
- **Named exports** for utilities and types
- **Barrel exports** (index.ts) for feature modules
- **Relative imports** within features, absolute for cross-feature

### Component Patterns
- **Functional components** with SolidJS
- **Props destructuring** with clear types
- **Early returns** for conditional rendering
- **Resource pattern** for async data fetching
