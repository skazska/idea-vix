import { query } from "@solidjs/router";
import type { WorkshopItem, NewWorkshopItem, PatchWorkshopItem, WorkshopItemType } from "./model";
import { getResponse, type IBackend } from "../providers/backend";
import type { ICrudApi } from "../crud/model";

// Entity type for workshop APIs (package or board)
export type WorkshopEntityType = 'package' | 'board';

// Workshop API interface for a specific item type within an entity
export interface IWorkshopApi {
    list: (entityId: string) => Promise<WorkshopItem[]>;
    create: (entityId: string, item: NewWorkshopItem) => Promise<WorkshopItem>;
    update: (entityId: string, itemId: string, item: PatchWorkshopItem) => Promise<WorkshopItem>;
    remove: (entityId: string, itemId: string) => Promise<boolean>;
}

// Create workshop API for a specific entity type and item type
export function createWorkshopApi(
    backend: IBackend, 
    entityType: WorkshopEntityType, 
    itemType: WorkshopItemType
): IWorkshopApi {
    const basePath = `/api/${entityType}`;
    
    return {
        async list(entityId: string): Promise<WorkshopItem[]> {
            const url = `${basePath}/${entityId}/workshop/${itemType}`;
            return getResponse(
                backend.fetchJson(url), 
                (data) => data as WorkshopItem[], 
                url
            );
        },

        async create(entityId: string, item: NewWorkshopItem): Promise<WorkshopItem> {
            const url = `${basePath}/${entityId}/workshop/${itemType}`;
            return getResponse(
                backend.fetchJson(url, {
                    method: 'POST',
                    body: JSON.stringify(item),
                    headers: { 'Content-Type': 'application/json' }
                }), 
                (data) => data as WorkshopItem, 
                url
            );
        },

        async update(entityId: string, itemId: string, item: PatchWorkshopItem): Promise<WorkshopItem> {
            const url = `${basePath}/${entityId}/workshop/${itemType}/${itemId}`;
            return getResponse(
                backend.fetchJson(url, {
                    method: 'PUT',
                    body: JSON.stringify(item),
                    headers: { 'Content-Type': 'application/json' }
                }), 
                (data) => data as WorkshopItem, 
                url
            );
        },

        async remove(entityId: string, itemId: string): Promise<boolean> {
            const url = `${basePath}/${entityId}/workshop/${itemType}/${itemId}`;
            return getResponse(
                backend.fetchJson(url, { method: 'DELETE' }), 
                () => true, 
                url
            );
        }
    };
}

// Create cached workshop API functions using SolidJS query
export function createCachedWorkshopApi(
    _backend: IBackend, 
    entityType: WorkshopEntityType, 
    itemType: WorkshopItemType
): ICrudApi<WorkshopItem, NewWorkshopItem, PatchWorkshopItem> {
    return {
        list: query(async () => {
            throw new Error("Workshop list requires entityId - use workshop provider instead");
        }, `workshop-${entityType}-${itemType}-list`),

        get: query(async (_id: string) => {
            throw new Error("Workshop get by ID not supported - items are fetched via list");
        }, `workshop-${entityType}-${itemType}-get`),

        create: query(async (_item: NewWorkshopItem) => {
            throw new Error("Workshop create requires entityId - use workshop provider instead");
        }, `workshop-${entityType}-${itemType}-create`),

        update: query(async (_id: string, _item: PatchWorkshopItem) => {
            throw new Error("Workshop update requires entityId - use workshop provider instead");
        }, `workshop-${entityType}-${itemType}-update`),

        remove: query(async (_id: string) => {
            throw new Error("Workshop remove requires entityId - use workshop provider instead");
        }, `workshop-${entityType}-${itemType}-remove`)
    };
}

// Workshop API factory for specific entity and item type with entityId bound
export interface IEntityWorkshopApi {
    list: () => Promise<WorkshopItem[]>;
    create: (item: NewWorkshopItem) => Promise<WorkshopItem>;
    update: (itemId: string, item: PatchWorkshopItem) => Promise<WorkshopItem>;
    remove: (itemId: string) => Promise<boolean>;
}

export function createEntityWorkshopApi(
    backend: IBackend,
    entityType: WorkshopEntityType,
    itemType: WorkshopItemType, 
    entityId: string
): IEntityWorkshopApi {
    const workshopApi = createWorkshopApi(backend, entityType, itemType);
    
    return {
        list: () => workshopApi.list(entityId),
        create: (item: NewWorkshopItem) => workshopApi.create(entityId, item),
        update: (itemId: string, item: PatchWorkshopItem) => workshopApi.update(entityId, itemId, item),
        remove: (itemId: string) => workshopApi.remove(entityId, itemId)
    };
}

// Global workshop API for read-only discovery
export interface IGlobalWorkshopApi {
    list: (itemType: WorkshopItemType, filters?: { ids?: number[] }) => Promise<WorkshopItem[]>;
    get: (itemType: WorkshopItemType, id: string) => Promise<WorkshopItem>;
    getBySlug: (itemType: WorkshopItemType, slug: string) => Promise<WorkshopItem>;
}

export function createGlobalWorkshopApi(backend: IBackend): IGlobalWorkshopApi {
    return {
        async list(itemType: WorkshopItemType, filters?: { ids?: number[] }): Promise<WorkshopItem[]> {
            let url = `/api/workshop/${itemType}`;
            if (filters?.ids && filters.ids.length > 0) {
                const params = new URLSearchParams();
                filters.ids.forEach(id => params.append('ids', id.toString()));
                url += `?${params.toString()}`;
            }
            return getResponse(
                backend.fetchJson(url), 
                (data) => data as WorkshopItem[], 
                url
            );
        },

        async get(itemType: WorkshopItemType, id: string): Promise<WorkshopItem> {
            const url = `/api/workshop/${itemType}/${id}`;
            return getResponse(
                backend.fetchJson(url), 
                (data) => data as WorkshopItem, 
                url
            );
        },

        async getBySlug(itemType: WorkshopItemType, slug: string): Promise<WorkshopItem> {
            const url = `/api/workshop/${itemType}/by-slug/${slug}`;
            return getResponse(
                backend.fetchJson(url), 
                (data) => data as WorkshopItem, 
                url
            );
        }
    };
}