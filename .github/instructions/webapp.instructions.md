---
description: Frontend (SolidJS/TypeScript) specific instructions
---
applyTo: "ui/web/**"
---

# Frontend Specific Instructions

SolidJS/TypeScript/Tailwind/Vite/Playwright/@modular-forms/valibot.
Type safety.
API types/interfaces.
Typed components.

`ui/web/`
`npm run dev` (http://localhost:5173)


backend calls:`common/providers/backend.tsx`
`package|board/const.ts` - constants
`package|board/providers/api.ts` - API
`package|board/providers/provider.tsx` - data provider

Component Classes: root, feature, utility, service.
Component Types: visual, provider, API-service, generator.
Component compositions.
File-based routing: with lazy-loaded components.
Preload api functions.

Session management through backend provider.
No global state library - use SolidJS context and providers.

Form Handling: Define schema, create form, bind to inputs.
Type safety: Infer types from valibot schemas.

Component Conventions:
- Composition over inheritance
- Props interfaces with clear naming
- Responsive design with Tailwind CSS classes
- Lazy loading for route components
- Small, focused components over large ones

TypeScript:
- Strict mode
- Interface over type
- Explicit return types
- Const assertions

Import/Export:
Default exports for components.
Named exports for others.
index.ts for feature modules.
Relative imports within features, absolute for cross-feature.

Component Patterns;
- Functional components
- Props destructuring with clear types
- Early returns for conditional rendering
- Resource pattern for async data fetching
