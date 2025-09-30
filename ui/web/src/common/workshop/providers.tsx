import { createContext, useContext } from "solid-js";
import { query, revalidate, createAsync, type AccessorWithLatest } from "@solidjs/router";
import type { ParentComponent } from "solid-js";
import type { WorkshopItem, NewWorkshopItem, PatchWorkshopItem, WorkshopItemType } from "./model";
import type { WorkshopEntityType } from "./api";
import { createWorkshopApi } from "./api";
import { useBackend } from "../providers/backend";

// Extended workshop item with type information
export interface WorkshopItemWithType extends WorkshopItem {
    item_type: WorkshopItemType;
}

// Workshop items context for all types within an entity
export interface IWorkshopManagerContext {
    allItems: AccessorWithLatest<WorkshopItemWithType[] | undefined>;
    itemsByType: (type: WorkshopItemType) => WorkshopItemWithType[];
    create: (itemType: WorkshopItemType, item: NewWorkshopItem) => Promise<WorkshopItem>;
    update: (itemType: WorkshopItemType, itemId: string, item: PatchWorkshopItem) => Promise<WorkshopItem>;
    remove: (itemType: WorkshopItemType, itemId: string) => Promise<boolean>;
    reload: () => void;
}

const WorkshopManagerContext = createContext<IWorkshopManagerContext>();

export interface WorkshopManagerProviderProps {
    entityType: WorkshopEntityType;
    entityId: string;
}

export const WorkshopManagerProvider: ParentComponent<WorkshopManagerProviderProps> = (props) => {
    const backend = useBackend();
    
    // Create APIs for all item types
    const apis = {
        shapes: createWorkshopApi(backend, props.entityType, "shapes"),
        lines: createWorkshopApi(backend, props.entityType, "lines"),
        rules: createWorkshopApi(backend, props.entityType, "rules"),
        layouts: createWorkshopApi(backend, props.entityType, "layouts"),
    };
    
    // Create query function that loads all workshop items
    const allItemsQuery = query(async () => {
        try {
            const results = await Promise.all([
                apis.shapes.list(props.entityId).then(items => 
                    items.map(item => ({ ...item, item_type: "shapes" as WorkshopItemType }))
                ),
                apis.lines.list(props.entityId).then(items => 
                    items.map(item => ({ ...item, item_type: "lines" as WorkshopItemType }))
                ),
                apis.rules.list(props.entityId).then(items => 
                    items.map(item => ({ ...item, item_type: "rules" as WorkshopItemType }))
                ),
                apis.layouts.list(props.entityId).then(items => 
                    items.map(item => ({ ...item, item_type: "layouts" as WorkshopItemType }))
                ),
            ]);
            
            return results.flat();
        } catch (error) {
            console.error(`Failed to load workshop items for ${props.entityType} ${props.entityId}:`, error);
            return [];
        }
    }, `workshop-${props.entityType}-${props.entityId}-all`);
    
    // Create async resource for all items
    const allItems = createAsync(() => allItemsQuery(), {
        name: `workshop-${props.entityType}-${props.entityId}-all-items`
    });
    
    const itemsByType = (type: WorkshopItemType): WorkshopItemWithType[] => {
        return allItems()?.filter(item => item.item_type === type) || [];
    };
    
    const create = async (itemType: WorkshopItemType, item: NewWorkshopItem): Promise<WorkshopItem> => {
        const result = await apis[itemType].create(props.entityId, item);
        reload(); // Refresh the list
        return result;
    };
    
    const update = async (itemType: WorkshopItemType, itemId: string, item: PatchWorkshopItem): Promise<WorkshopItem> => {
        const result = await apis[itemType].update(props.entityId, itemId, item);
        reload(); // Refresh the list
        return result;
    };
    
    const remove = async (itemType: WorkshopItemType, itemId: string): Promise<boolean> => {
        const result = await apis[itemType].remove(props.entityId, itemId);
        reload(); // Refresh the list
        return result;
    };
    
    const reload = () => {
        // Trigger revalidation of all items
        const key = allItemsQuery.keyFor();
        console.log(`Reloading all workshop items with key:`, key);
        revalidate(key);
    };
    
    const contextValue: IWorkshopManagerContext = {
        allItems,
        itemsByType,
        create,
        update,
        remove,
        reload
    };
    
    return (
        <WorkshopManagerContext.Provider value={contextValue}>
            {props.children}
        </WorkshopManagerContext.Provider>
    );
};

export function useWorkshopManager(): IWorkshopManagerContext {
    const context = useContext(WorkshopManagerContext);
    if (!context) {
        throw new Error("useWorkshopManager must be used within a WorkshopManagerProvider");
    }
    return context;
}

// Keep the original single-type provider for backward compatibility
export interface IWorkshopItemsContext {
    items: AccessorWithLatest<WorkshopItem[] | undefined>;
    create: (item: NewWorkshopItem) => Promise<WorkshopItem>;
    update: (itemId: string, item: PatchWorkshopItem) => Promise<WorkshopItem>;
    remove: (itemId: string) => Promise<boolean>;
    reload: () => void;
}

const WorkshopItemsContext = createContext<IWorkshopItemsContext>();

export interface WorkshopItemsProviderProps {
    entityType: WorkshopEntityType;
    entityId: string;
    itemType: WorkshopItemType;
}

export const WorkshopItemsProvider: ParentComponent<WorkshopItemsProviderProps> = (props) => {
    const backend = useBackend();
    const api = createWorkshopApi(backend, props.entityType, props.itemType);
    
    // Create cached list query function
    const listQuery = query(async () => {
        try {
            return await api.list(props.entityId);
        } catch (error) {
            console.error(`Failed to load ${props.itemType} for ${props.entityType} ${props.entityId}:`, error);
            return [];
        }
    }, `workshop-${props.entityType}-${props.entityId}-${props.itemType}-list`);
    
    // Create async resource for items list
    const items = createAsync(() => listQuery(), {
        name: `workshop-${props.entityType}-${props.entityId}-${props.itemType}-items`
    });
    
    const create = async (item: NewWorkshopItem): Promise<WorkshopItem> => {
        const result = await api.create(props.entityId, item);
        reload(); // Refresh the list
        return result;
    };
    
    const update = async (itemId: string, item: PatchWorkshopItem): Promise<WorkshopItem> => {
        const result = await api.update(props.entityId, itemId, item);
        reload(); // Refresh the list
        return result;
    };
    
    const remove = async (itemId: string): Promise<boolean> => {
        const result = await api.remove(props.entityId, itemId);
        reload(); // Refresh the list
        return result;
    };
    
    const reload = () => {
        // Trigger revalidation of the items
        const key = listQuery.keyFor();
        console.log(`Reloading workshop ${props.itemType} with key:`, key);
        revalidate(key);
    };
    
    const contextValue: IWorkshopItemsContext = {
        items,
        create,
        update,
        remove,
        reload
    };
    
    return (
        <WorkshopItemsContext.Provider value={contextValue}>
            {props.children}
        </WorkshopItemsContext.Provider>
    );
};

export function useWorkshopItems(): IWorkshopItemsContext {
    const context = useContext(WorkshopItemsContext);
    if (!context) {
        throw new Error("useWorkshopItems must be used within a WorkshopItemsProvider");
    }
    return context;
}