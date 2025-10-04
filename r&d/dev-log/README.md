# Dev log

## Purpose

This is the root of the dev log. It contains links to all other dev iteration logs.

## Structure and Usage

[dev log index file](index.md) contains:

- name
- date (YY-MM-DD)
- purpose
- status (draft, in-progress, completed)
- link to the iteration's details files

Iteration details are in one md file or a set of files in folder named, either way named `YY-MM-DD-{name}`.
Types of details: definition, research, plan, implementation, testing, status.
If in one md file, Details to be placed in sections with headings named after the types.
If in a folder, each type to be in its own file named `type.md`.

### Definition

Developer must initiate iteration and compose draft at least.

Content:

- purpose and goals (MUST)
- definition of done (MUST)
- description of the feature or change (MUST)
- references to focus documents, code, data, documentation (SHOULD)
- requirements and specifications (SHOULD)
- any problems, open questions, uncertainties, challenges anticipated (COULD)
- any relevant diagrams or illustrations (COULD)

### Research

To register results of research or analysis related to the iteration:

- analysis of definition content
- code, data, documentation stressed or referenced in definition or project documentation in `dev-docs` and `analysis` folders
- insights, findings, conclusions to be used for implementation with references.
- any weak points, lack of clarity, requests for additional information.

### Plan

To outline the approach, tasks, and description of what, how and why to be done: changing, adding, removing, actualizing tests, code, data, documentation, other artifacts.

### Implementation

To log the actual implementation details and progress according to the plan:

- tasks with status
- any deviations from the plan with reasons
- any problems, issues, blockers encountered
- references to code changes, commits, pull requests

### Testing

To log the testing process and decisions on done according to the definition of done or goals.

To log any problems, issues encountered which is not related to iteration.
