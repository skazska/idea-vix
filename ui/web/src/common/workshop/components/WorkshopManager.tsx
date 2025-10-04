import { createSignal, Show, For } from "solid-js";
import { Plus, Eye, Edit, Trash2 } from "lucide-solid";
import type { WorkshopItemType, NewWorkshopItem, PatchWorkshopItem } from "../model";
import type { WorkshopItemWithType } from "../providers";
import { getWorkshopItemTypeDisplayName } from "../model";
import { useWorkshopManager } from "../providers";
import { WorkshopItemForm } from "./";

export interface WorkshopManagerProps {
    entityType: "package" | "board";
    entityId: string;
}

type FormMode = "add" | "edit" | null;

export function WorkshopManager(_props: WorkshopManagerProps) {
    const workshopContext = useWorkshopManager();
    const [selectedItemType, setSelectedItemType] = createSignal<WorkshopItemType>("shapes");
    const [formMode, setFormMode] = createSignal<FormMode>(null);
    const [editingItem, setEditingItem] = createSignal<WorkshopItemWithType | undefined>();
    const [isLoading, setIsLoading] = createSignal(false);
    
    if (!workshopContext) {
        return <div class="text-red-500">Workshop context not available</div>;
    }
    
    const { itemsByType, create, update, remove } = workshopContext;
    const itemTypes: WorkshopItemType[] = ["shapes", "lines", "rules", "layouts"];
    
    const itemsForType = (type: WorkshopItemType) => {
        return itemsByType(type);
    };
    
    const openAddForm = (itemType: WorkshopItemType) => {
        setSelectedItemType(itemType);
        setEditingItem(undefined);
        setFormMode("add");
    };
    
    const openEditForm = (item: WorkshopItemWithType) => {
        setSelectedItemType(item.item_type);
        setEditingItem(item);
        setFormMode("edit");
    };
    
    const closeForm = () => {
        setFormMode(null);
        setEditingItem(undefined);
        setIsLoading(false);
    };
    
    const handleSave = async (data: NewWorkshopItem | PatchWorkshopItem) => {
        setIsLoading(true);
        
        try {
            if (formMode() === "add") {
                await create(selectedItemType(), data as NewWorkshopItem);
            } else if (formMode() === "edit" && editingItem()) {
                await update(editingItem()!.item_type, editingItem()!.id.toString(), data as PatchWorkshopItem);
            }
            closeForm();
        } catch (error) {
            console.error("Failed to save workshop item:", error);
            setIsLoading(false);
            // Re-throw the error so the form component can handle it
            throw error;
        }
    };
    
    const handleDelete = async (item: WorkshopItemWithType) => {
        if (confirm(`Are you sure you want to delete "${item.name}"?`)) {
            await remove(item.item_type, item.id.toString());
        }
    };
    
    const handleView = (item: WorkshopItemWithType) => {
        // TODO: Implement view/preview functionality
        console.log("View item:", item);
        alert(`View functionality for "${item.name}" will be implemented later.`);
    };
    
    return (
        <div class="space-y-6" data-testid="workshop-manager">
            {/* Header */}
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-bold">Workshop Items</h2>
            </div>
            
            {/* Item Type Tabs */}
            <div class="border-b border-gray-200">
                <nav class="-mb-px flex space-x-8">
                    <For each={itemTypes}>
                        {(itemType) => (
                            <button
                                class={`py-2 px-1 border-b-2 font-medium text-sm ${
                                    selectedItemType() === itemType
                                        ? 'border-blue-500 text-blue-600'
                                        : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                                }`}
                                onClick={() => setSelectedItemType(itemType)}
                                data-testid={`workshop-tab-${itemType}`}
                            >
                                {getWorkshopItemTypeDisplayName(itemType)}
                                <span class="ml-2 bg-gray-100 text-gray-600 text-xs px-2 py-0.5 rounded-full">
                                    {itemsForType(itemType).length}
                                </span>
                            </button>
                        )}
                    </For>
                </nav>
            </div>
            
            {/* Content for selected type */}
            <div class="space-y-4">
                {/* Add button */}
                <div class="flex justify-between items-center">
                    <h3 class="text-lg font-medium">
                        {getWorkshopItemTypeDisplayName(selectedItemType())}
                    </h3>
                    <button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded flex items-center gap-2"
                        onClick={() => openAddForm(selectedItemType())}
                        data-testid={`workshop-add-${selectedItemType()}-button`}
                    >
                        <Plus size="1rem" />
                        Add {getWorkshopItemTypeDisplayName(selectedItemType()).slice(0, -1)}
                    </button>
                </div>
                
                {/* Items list */}
                <div class="space-y-2">
                    <Show when={itemsForType(selectedItemType()).length === 0}>
                        <div class="text-gray-500 text-center py-8">
                            No {getWorkshopItemTypeDisplayName(selectedItemType()).toLowerCase()} found.
                            <br />
                            <span class="text-sm">Click "Add" to create your first one.</span>
                        </div>
                    </Show>
                    
                    <For each={itemsForType(selectedItemType())}>
                        {(item) => (
                            <div class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50" data-testid={`workshop-item-${item.slug}`}>
                                <div class="flex justify-between items-start">
                                    <div class="flex-1">
                                        <h4 class="font-medium text-gray-900">{item.name}</h4>
                                        <p class="text-sm text-gray-500 font-mono">{item.slug}</p>
                                        <Show when={item.description}>
                                            <p class="text-sm text-gray-600 mt-1">{item.description}</p>
                                        </Show>
                                        <div class="text-xs text-gray-400 mt-2">
                                            Created: {new Date(item.created_at * 1000).toLocaleDateString()}
                                            {item.updated_at !== item.created_at && (
                                                <span> • Updated: {new Date(item.updated_at * 1000).toLocaleDateString()}</span>
                                            )}
                                        </div>
                                    </div>
                                    
                                    <div class="flex gap-1 ml-4">
                                        <button
                                            class="text-blue-600 hover:text-blue-800 p-1"
                                            onClick={() => handleView(item)}
                                            title="View item"
                                            data-testid={`workshop-view-${item.slug}`}
                                        >
                                            <Eye size="1rem" />
                                        </button>
                                        <button
                                            class="text-green-600 hover:text-green-800 p-1"
                                            onClick={() => openEditForm(item)}
                                            title="Edit item"
                                            data-testid={`workshop-edit-${item.slug}`}
                                        >
                                            <Edit size="1rem" />
                                        </button>
                                        <button
                                            class="text-red-600 hover:text-red-800 p-1"
                                            onClick={() => handleDelete(item)}
                                            title="Delete item"
                                            data-testid={`workshop-delete-${item.slug}`}
                                        >
                                            <Trash2 size="1rem" />
                                        </button>
                                    </div>
                                </div>
                            </div>
                        )}
                    </For>
                </div>
            </div>
            
            {/* Modal for add/edit form */}
            <Show when={formMode()}>
                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
                    <div class="bg-white rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto">
                        <WorkshopItemForm
                            itemType={selectedItemType()}
                            item={editingItem()}
                            onSave={handleSave}
                            onCancel={closeForm}
                            isLoading={isLoading()}
                        />
                    </div>
                </div>
            </Show>
        </div>
    );
}