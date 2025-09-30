import { createSignal, Show, onMount } from "solid-js";
import { Save, X } from "lucide-solid";
import type { WorkshopItem, NewWorkshopItem, PatchWorkshopItem, WorkshopItemType } from "../model";
import { getWorkshopItemTypeDisplayName, validateWorkshopItemDefinition } from "../model";

export interface WorkshopItemFormProps {
    itemType: WorkshopItemType;
    item?: WorkshopItem; // If provided, this is an edit form
    onSave: (item: NewWorkshopItem | PatchWorkshopItem) => Promise<void>;
    onCancel: () => void;
    isLoading?: boolean;
}

export function WorkshopItemForm(props: WorkshopItemFormProps) {
    const [name, setName] = createSignal("");
    const [slug, setSlug] = createSignal("");
    const [description, setDescription] = createSignal("");
    const [definitionJson, setDefinitionJson] = createSignal("");
    const [errors, setErrors] = createSignal<Record<string, string>>({});
    
    const displayName = () => getWorkshopItemTypeDisplayName(props.itemType);
    const isEdit = () => !!props.item;
    
    // Initialize form with existing data if editing
    onMount(() => {
        if (props.item) {
            setName(props.item.name);
            setSlug(props.item.slug);
            setDescription(props.item.description || "");
            setDefinitionJson(JSON.stringify(props.item.definition, null, 2));
        } else {
            // Set default definition template based on item type
            const defaultDefinitions = {
                shapes: {
                    background: {
                        border: {
                            path: "M0,0 L100,0 L100,50 L0,50 Z",
                            stroke: { width: 2, color: "#000000" }
                        }
                    },
                    label: {
                        position: { x: 50, y: 25 },
                        default_text: "Shape"
                    }
                },
                lines: {
                    stroke: {
                        width: 2,
                        color: "#000000",
                        style: "solid"
                    },
                    source_socket: "output",
                    target_socket: "input"
                },
                rules: {
                    conditions: {},
                    actions: {}
                },
                layouts: {
                    type: "grid",
                    properties: {}
                }
            };
            
            setDefinitionJson(JSON.stringify(defaultDefinitions[props.itemType], null, 2));
        }
    });
    
    // Generate slug from name when creating new items
    const generateSlug = (name: string) => {
        return name
            .toLowerCase()
            .replace(/[^a-z0-9\s-]/g, '')
            .replace(/\s+/g, '-')
            .replace(/-+/g, '-')
            .replace(/^-+|-+$/g, '');
    };
    
    const handleNameChange = (newName: string) => {
        setName(newName);
        // Auto-generate slug only for new items
        if (!isEdit()) {
            setSlug(generateSlug(newName));
        }
    };
    
    const validate = (): boolean => {
        const newErrors: Record<string, string> = {};
        
        if (!name().trim()) {
            newErrors.name = "Name is required";
        } else if (name().length > 100) {
            newErrors.name = "Name must be at most 100 characters";
        }
        
        if (!isEdit()) { // Slug validation only for new items
            if (!slug().trim()) {
                newErrors.slug = "Slug is required";
            } else if (slug().length > 100) {
                newErrors.slug = "Slug must be at most 100 characters";
            } else if (!/^[a-z][a-z0-9]*(-[a-z0-9]+)*$/.test(slug())) {
                newErrors.slug = "Slug must start with a letter, contain only lowercase letters, numbers, and hyphens";
            }
        }
        
        if (description().length > 500) {
            newErrors.description = "Description must be at most 500 characters";
        }
        
        if (!definitionJson().trim()) {
            newErrors.definition = "Definition is required";
        } else {
            try {
                const definition = JSON.parse(definitionJson());
                if (!validateWorkshopItemDefinition(props.itemType, definition)) {
                    newErrors.definition = `Invalid ${props.itemType} definition format`;
                }
            } catch {
                newErrors.definition = "Definition must be valid JSON";
            }
        }
        
        setErrors(newErrors);
        return Object.keys(newErrors).length === 0;
    };
    
    const handleSave = async () => {
        if (!validate()) return;
        
        try {
            const definition = JSON.parse(definitionJson());
            
            if (isEdit()) {
                const patchData: PatchWorkshopItem = {
                    name: name() !== props.item!.name ? name() : undefined,
                    description: description() !== (props.item!.description || "") 
                        ? (description() || undefined) : undefined,
                    definition: JSON.stringify(definition) !== JSON.stringify(props.item!.definition) 
                        ? definition : undefined
                };
                
                // Only include fields that actually changed
                const hasChanges = Object.values(patchData).some(value => value !== undefined);
                if (hasChanges) {
                    await props.onSave(patchData);
                } else {
                    props.onCancel(); // No changes to save
                }
            } else {
                const newData: NewWorkshopItem = {
                    name: name(),
                    slug: slug(),
                    description: description() || undefined,
                    definition
                };
                
                await props.onSave(newData);
            }
        } catch (error) {
            console.error("Failed to save workshop item:", error);
            setErrors({ save: "Failed to save. Please try again." });
        }
    };
    
    return (
        <div class="bg-white rounded-lg shadow p-6">
            <div class="flex justify-between items-center mb-6">
                <h2 class="text-xl font-bold">
                    {isEdit() ? `Edit ${displayName()}` : `Add New ${displayName()}`}
                </h2>
                <div class="flex gap-2">
                    <button
                        class="bg-green-500 hover:bg-green-700 disabled:bg-gray-400 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1"
                        onClick={handleSave}
                        disabled={props.isLoading}
                        data-testid="workshop-form-save-button"
                    >
                        <Save size="0.8rem" />
                        Save
                    </button>
                    <button
                        class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1"
                        onClick={props.onCancel}
                        disabled={props.isLoading}
                        data-testid="workshop-form-cancel-button"
                    >
                        <X size="0.8rem" />
                        Cancel
                    </button>
                </div>
            </div>
            
            <div class="space-y-4">
                {/* Name field */}
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Name *
                    </label>
                    <input
                        type="text"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={name()}
                        onInput={(e) => handleNameChange(e.target.value)}
                        placeholder="Enter a descriptive name"
                        data-testid="workshop-form-name-input"
                    />
                    <Show when={errors().name}>
                        <p class="text-red-500 text-sm mt-1">{errors().name}</p>
                    </Show>
                </div>
                
                {/* Slug field - only editable for new items */}
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Slug * {isEdit() && <span class="text-gray-500">(cannot be changed)</span>}
                    </label>
                    <input
                        type="text"
                        class={`w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 ${isEdit() ? 'bg-gray-100 text-gray-600' : ''}`}
                        value={slug()}
                        onInput={(e) => setSlug(e.target.value)}
                        placeholder="lowercase-with-hyphens"
                        disabled={isEdit()}
                        data-testid="workshop-form-slug-input"
                    />
                    <Show when={errors().slug}>
                        <p class="text-red-500 text-sm mt-1">{errors().slug}</p>
                    </Show>
                    <Show when={!isEdit()}>
                        <p class="text-gray-500 text-sm mt-1">
                            Unique identifier, automatically generated from name
                        </p>
                    </Show>
                </div>
                
                {/* Description field */}
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Description
                    </label>
                    <textarea
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        rows="3"
                        value={description()}
                        onInput={(e) => setDescription(e.target.value)}
                        placeholder="Optional description"
                        data-testid="workshop-form-description-input"
                    />
                    <Show when={errors().description}>
                        <p class="text-red-500 text-sm mt-1">{errors().description}</p>
                    </Show>
                </div>
                
                {/* Definition field */}
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Definition (JSON) *
                    </label>
                    <textarea
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-sm"
                        rows="12"
                        value={definitionJson()}
                        onInput={(e) => setDefinitionJson(e.target.value)}
                        placeholder="Enter JSON definition..."
                        data-testid="workshop-form-definition-input"
                    />
                    <Show when={errors().definition}>
                        <p class="text-red-500 text-sm mt-1">{errors().definition}</p>
                    </Show>
                    <p class="text-gray-500 text-sm mt-1">
                        JSON definition for {props.itemType} properties and appearance
                    </p>
                </div>
                
                {/* General save error */}
                <Show when={errors().save}>
                    <p class="text-red-500 text-sm">{errors().save}</p>
                </Show>
                
                <Show when={props.isLoading}>
                    <p class="text-blue-500 text-sm">Saving...</p>
                </Show>
            </div>
        </div>
    );
}