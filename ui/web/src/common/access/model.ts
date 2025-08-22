import { createAsync, revalidate, type AccessorWithLatest, type CachedFunction } from "@solidjs/router";

export type AccessRole = 'owner' | 'manage' | 'edit' | 'view';

export type GrantRequest = {
  address: string;
  role: Exclude<AccessRole, 'owner'>; // cannot grant owner via UI
};

export type AccessMapping = {
  // resource id field name differs per entity; we don't strictly depend on it in UI
  address: string;
  role: AccessRole | string; // backend returns string; normalize for display
};

export const grantableRoles: GrantRequest['role'][] = ['view', 'edit', 'manage'];


export type TAccessMapModel = [
    
    AccessorWithLatest<AccessMapping[] | undefined>,
    {
        grant: CachedFunction<(id: string, item: GrantRequest) => Promise<AccessMapping>>;
        revoke: CachedFunction<(id: string, item: string) => Promise<AccessMapping>>;
        reload: () => void;
    }
];

export interface IAccessMapApi {
    listAccess: CachedFunction<(id: string) => Promise<AccessMapping[]>>;
    grantAccess: CachedFunction<(id: string, item: GrantRequest) => Promise<AccessMapping>>;
    revokeAccess: CachedFunction<(id: string, item: string) => Promise<AccessMapping>>;
}

export function getAccessMapModel(api: IAccessMapApi, id: string): TAccessMapModel {
    const { grantAccess, revokeAccess } = api;
    // const list = createAsync(() => api.listAccess(props.id), { 
    //     name: `${props.entity}-item-invitations`,
    // });
    const list = createAsync(() => api.listAccess(id));

    console.log("BoardAccessListProvider rendered for board:", id);

    return [
        list,
        {
            grant: grantAccess.bind(api),
            revoke: revokeAccess.bind(api),
            reload: () => {
                const key = api.listAccess.keyFor(id);
                console.log("Reloading access list with revalidate key:", key);
                revalidate(key);
            }
        }
    ];
}

export interface IAccessApi {
  myAccess: CachedFunction<(id: string) => Promise<AccessRole[]>>;
}

export type TAccessModel = [
    AccessorWithLatest<AccessRole[] | undefined>,
    {
        reload: () => void;
    }
];

export function getAccessModel(api: IAccessApi, id: string): TAccessModel {
    const access = createAsync(() => api.myAccess(id));
    // const list = createAsync(() => api.myAccess(id), { 
    //     name: `${entity}-item-invitations`,
    // });

    console.log("AccessProvider rendered for id:", id);

    return [
        access,
        {
            reload: () => {
                const key = api.myAccess.keyFor(id);
                console.log("Reloading access with revalidate key:", key);
                revalidate(key);
            }
        }
    ];
}

