import { createAsync, revalidate, type AccessorWithLatest, type CachedFunction } from "@solidjs/router";

export interface ICrudApi<T, N, U> {
    list: CachedFunction<() => Promise<T[]>>;
    get: CachedFunction<(id: string) => Promise<T>>;
    create: CachedFunction<(item: N) => Promise<T>>;
    update: CachedFunction<(id: string, item: U) => Promise<T>>;
    remove: CachedFunction<(id: string) => Promise<boolean>>;
}


export type TItemsModel<T, N> = [
    AccessorWithLatest<T[] | undefined >,
    {
        create: CachedFunction<(item: N) => Promise<T>>;
        remove: CachedFunction<(id: string) => Promise<boolean>>;
        reload: () => void;
    }
];

export function getItemsModel<T, N, U>(api: ICrudApi<T, N, U>): TItemsModel<T, N> {
    const { create, remove } = api;
    const list = createAsync(() => api.list());

    console.log("ItemsProvider rendered");

    return [
        list,
        {
            create: create.bind(api),
            remove: remove.bind(api),
            reload: () => {
                const key = api.list.keyFor();
                console.log("Reloading items with revalidate key:", key);
                revalidate(key);
            }
        }
    ];
}

export type TItemModel<T, U> = [
    AccessorWithLatest<T | undefined>,
    {
        remove: CachedFunction<(id: string) => Promise<boolean>>;
        update: CachedFunction<(id: string, item: U) => Promise<T>>;
        reload: () => void;
    }
];

export function getItemModel<T, U>(api: ICrudApi<T, never, U>, id: string): TItemModel<T, U> {
    const { update, remove } = api;
    
    // FIXME?
    const item: AccessorWithLatest<T | undefined> = createAsync(() => api.get(id)) as AccessorWithLatest<T | undefined>;

    // const item: AccessorWithLatest<T | undefined> = createAsync(() => api.get(id), { 
    //     name: `board-item-query`,
    // });

    console.log("ItemProvider rendered for item:", id);

    return [
        item,
        {
            remove: remove.bind(api),
            update: update.bind(api),
            reload: () => {
                const key = api.get.keyFor(id);
                console.log("Reloading item with revalidate key:", key);
                revalidate(key);
            }
        }
    ];
}
