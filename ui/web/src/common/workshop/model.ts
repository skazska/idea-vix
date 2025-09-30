import * as v from 'valibot';

// Common workshop item fields
const workshopItemFields = {
    id: v.number(),
    name: v.pipe(v.string(), v.nonEmpty('Name is required'), v.maxLength(100, 'Name must be at most 100 characters')),
    slug: v.pipe(
        v.string(), 
        v.nonEmpty('Slug is required'),
        v.maxLength(100, 'Slug must be at most 100 characters'),
        v.check((value) => /^[a-z][a-z0-9]*(-[a-z0-9]+)*$/.test(value), 'Slug must start with a letter, contain only lowercase letters, numbers, and hyphens')
    ),
    description: v.optional(v.pipe(v.string(), v.maxLength(500, 'Description must be at most 500 characters'))),
    definition: v.any(), // JSON definition - validate structure based on item type
    created_at: v.number(),
    updated_at: v.number(),
}

const newWorkshopItemFields = {
    name: v.pipe(v.string(), v.nonEmpty('Name is required'), v.maxLength(100, 'Name must be at most 100 characters')),
    slug: v.pipe(
        v.string(), 
        v.nonEmpty('Slug is required'),
        v.maxLength(100, 'Slug must be at most 100 characters'),
        v.check((value) => /^[a-z][a-z0-9]*(-[a-z0-9]+)*$/.test(value), 'Slug must start with a letter, contain only lowercase letters, numbers, and hyphens')
    ),
    description: v.optional(v.pipe(v.string(), v.maxLength(500, 'Description must be at most 500 characters'))),
    definition: v.any(), // JSON definition
}

const patchWorkshopItemFields = {
    name: v.optional(v.pipe(v.string(), v.nonEmpty('Name is required'), v.maxLength(100, 'Name must be at most 100 characters'))),
    description: v.optional(v.optional(v.pipe(v.string(), v.maxLength(500, 'Description must be at most 500 characters')))),
    definition: v.optional(v.any()), // JSON definition
}

// Workshop item schemas
export const WorkshopItemSchema = v.object(workshopItemFields);
export const NewWorkshopItemSchema = v.object(newWorkshopItemFields);
export const PatchWorkshopItemSchema = v.object(patchWorkshopItemFields);

// Workshop item types
export type WorkshopItem = v.InferOutput<typeof WorkshopItemSchema>;
export type NewWorkshopItem = v.InferOutput<typeof NewWorkshopItemSchema>;
export type PatchWorkshopItem = v.InferOutput<typeof PatchWorkshopItemSchema>;

// Workshop item types enum
export type WorkshopItemType = 'shapes' | 'lines' | 'rules' | 'layouts';

// Shape-specific definition schema (basic structure - can be extended)
export const ShapeDefinitionSchema = v.object({
    background: v.optional(v.object({
        border: v.optional(v.object({
            path: v.string(),
            stroke: v.optional(v.object({
                width: v.number(),
                color: v.string()
            })),
            fill: v.optional(v.string())
        }))
    })),
    label: v.optional(v.object({
        position: v.optional(v.object({
            x: v.number(),
            y: v.number()
        })),
        default_text: v.optional(v.string()),
        styling: v.optional(v.any())
    })),
    content: v.optional(v.any()),
    connection_sockets: v.optional(v.array(v.object({
        shape: v.optional(v.string()),
        tags: v.optional(v.array(v.string())),
        allowed_positions: v.optional(v.array(v.string()))
    })))
});

// Line-specific definition schema
export const LineDefinitionSchema = v.object({
    stroke: v.optional(v.object({
        width: v.number(),
        color: v.string(),
        style: v.optional(v.string())
    })),
    source_socket: v.optional(v.string()),
    target_socket: v.optional(v.string())
});

// Rule-specific definition schema (basic - can be extended based on requirements)
export const RuleDefinitionSchema = v.object({
    conditions: v.optional(v.any()),
    actions: v.optional(v.any())
});

// Layout-specific definition schema (basic - can be extended based on requirements)  
export const LayoutDefinitionSchema = v.object({
    type: v.optional(v.string()),
    properties: v.optional(v.any())
});

// Shape, Line, Rule, Layout specific types
export type ShapeDefinition = v.InferOutput<typeof ShapeDefinitionSchema>;
export type LineDefinition = v.InferOutput<typeof LineDefinitionSchema>;
export type RuleDefinition = v.InferOutput<typeof RuleDefinitionSchema>;
export type LayoutDefinition = v.InferOutput<typeof LayoutDefinitionSchema>;

// Helper function to get item type display name
export function getWorkshopItemTypeDisplayName(type: WorkshopItemType): string {
    switch (type) {
        case 'shapes': return 'Node Shape';
        case 'lines': return 'Connection Line';  
        case 'rules': return 'Connection Rule';
        case 'layouts': return 'Layout';
    }
}

// Helper function to get item type plural display name
export function getWorkshopItemTypePluralDisplayName(type: WorkshopItemType): string {
    switch (type) {
        case 'shapes': return 'Node Shapes';
        case 'lines': return 'Connection Lines';  
        case 'rules': return 'Connection Rules';
        case 'layouts': return 'Layouts';
    }
}

// Helper function to validate definition based on item type
export function validateWorkshopItemDefinition(type: WorkshopItemType, definition: any): boolean {
    try {
        switch (type) {
            case 'shapes':
                v.parse(ShapeDefinitionSchema, definition);
                return true;
            case 'lines':
                v.parse(LineDefinitionSchema, definition);
                return true;
            case 'rules':
                v.parse(RuleDefinitionSchema, definition);
                return true;
            case 'layouts':
                v.parse(LayoutDefinitionSchema, definition);
                return true;
            default:
                return false;
        }
    } catch {
        return false;
    }
}