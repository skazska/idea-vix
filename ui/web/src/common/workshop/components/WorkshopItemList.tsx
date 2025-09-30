import { Show, For, createSignal } from "solid-js";
import { Plus, Edit, Trash2, Eye } from "lucide-solid";
import type { WorkshopItem, WorkshopItemType } from "../model";
import { getWorkshopItemTypePluralDisplayName } from "../model";
import { useWorkshopItems } from "../providers";

export interface WorkshopItemListProps {
    itemType: WorkshopItemType;
    onAdd?: () => void;
    onEdit?: (item: WorkshopItem) => void;
    onView?: (item: WorkshopItem) => void;
    onDelete?: (item: WorkshopItem) => void;
    canManage?: boolean;
    canView?: boolean;
}

export function WorkshopItemList(props: WorkshopItemListProps) {
    const { items } = useWorkshopItems();
    const [isDeleting, setIsDeleting] = createSignal<number | null>(null);
    
    const displayName = () => getWorkshopItemTypePluralDisplayName(props.itemType);
    
    const handleDelete = async (item: WorkshopItem) => {
        if (!props.onDelete) return;
        
        setIsDeleting(item.id);
        try {
            await props.onDelete(item);
        } finally {
            setIsDeleting(null);
        }
    };
    
    return (
        <div class="bg-white rounded-lg shadow p-6">
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-xl font-bold">{displayName()}</h2>
                <Show when={props.canManage && props.onAdd}>
                    <button 
                        class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1"
                        onClick={props.onAdd}
                        data-testid={`add-${props.itemType}-button`}
                    >
                        <Plus size={'0.8rem'}/>
                        Add {getWorkshopItemTypePluralDisplayName(props.itemType).slice(0, -1)}
                    </button>
                </Show>
            </div>
            
            <Show 
                when={items() && items()!.length > 0} 
                fallback={
                    <div class="text-gray-500">
                        No {displayName().toLowerCase()} defined yet.
                    </div>
                }
            >
                <div class="space-y-2">
                    <For each={items()}>
                        {(item) => (
                            <div 
                                class="flex items-center justify-between p-3 border border-gray-200 rounded-lg hover:bg-gray-50"
                                data-testid={`workshop-item-${item.id}`}
                            >
                                <div class="flex-1">
                                    <div class="flex items-center gap-3">
                                        <div>
                                            <h3 class="font-medium text-gray-900">{item.name}</h3>
                                            <p class="text-sm text-gray-500 font-mono">{item.slug}</p>
                                            <Show when={item.description}>
                                                <p class="text-sm text-gray-600 mt-1">{item.description}</p>
                                            </Show>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="flex items-center gap-2">
                                    <Show when={props.canView && props.onView}>
                                        <button
                                            class="text-blue-600 hover:text-blue-800 p-1"
                                            onClick={() => props.onView!(item)}
                                            title="View details"
                                            data-testid={`view-${props.itemType}-${item.id}-button`}
                                        >
                                            <Eye size="1rem" />
                                        </button>
                                    </Show>
                                    
                                    <Show when={props.canManage && props.onEdit}>
                                        <button
                                            class="text-green-600 hover:text-green-800 p-1"
                                            onClick={() => props.onEdit!(item)}
                                            title="Edit"
                                            data-testid={`edit-${props.itemType}-${item.id}-button`}
                                        >
                                            <Edit size="1rem" />
                                        </button>
                                    </Show>
                                    
                                    <Show when={props.canManage && props.onDelete}>
                                        <button
                                            class="text-red-600 hover:text-red-800 p-1 disabled:opacity-50"
                                            onClick={() => handleDelete(item)}
                                            title="Delete"
                                            disabled={isDeleting() === item.id}
                                            data-testid={`delete-${props.itemType}-${item.id}-button`}
                                        >
                                            <Trash2 size="1rem" />
                                        </button>
                                    </Show>
                                </div>
                            </div>
                        )}
                    </For>
                </div>
            </Show>
        </div>
    );
}