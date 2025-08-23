/* @refresh reload */

import { query } from "@solidjs/router";
import type { TPackage, TPackageNew } from "../model";
import type { AccessMapping, AccessRole, GrantRequest, IAccessApi, IAccessMapApi } from "../../common/access/model";
import { getResponse, type IBackend } from "../../common/providers/backend";
import { REST_PATH, ENTITIES_NAME } from "../const";
import type { ICrudApi } from "../../common/crud/model";

export async function getPackages(backend: IBackend): Promise<TPackage[]> {
    return getResponse(backend.fetchJson(REST_PATH), (data) => data as TPackage[], REST_PATH);
}

export async function getPackage(backend: IBackend, id: string): Promise<TPackage> {
    return getResponse(backend.fetchJson(`${REST_PATH}/${id}`), (data) => data as TPackage, `${REST_PATH}/${id}`);
}

export async function addPackage(backend: IBackend, item: TPackageNew): Promise<TPackage> {
    return getResponse(backend.fetchJson(REST_PATH, {
        method: 'POST',
        body: JSON.stringify(item),
        headers: { 'Content-Type': 'application/json' }
    }), (data) => data as TPackage, REST_PATH);
}

export async function updatePackage(backend: IBackend, id: string, item: Partial<Omit<TPackage, 'id'>>): Promise<TPackage> {
    return getResponse(backend.fetchJson(`${REST_PATH}/${id}`, {
        method: 'PUT',
        body: JSON.stringify(item),
        headers: { 'Content-Type': 'application/json' }
    }), (data) => data as TPackage, `${REST_PATH}/${id}`);
}

export async function removePackage(backend: IBackend, id: string): Promise<boolean> {
    return getResponse(backend.fetchJson(`${REST_PATH}/${id}`, { method: 'DELETE' }), (_data) => true, `${REST_PATH}/${id}`);
}

// Access endpoints
export async function listPackageAccess(backend: IBackend, id: string): Promise<AccessMapping[]> {
    return getResponse(
        backend.fetchJson(`${REST_PATH}/${id}/access`),
        (data) => data as AccessMapping[],
        `${REST_PATH}/${id}/access`
    );
}

export async function grantPackageAccess(backend: IBackend, id: string, item: GrantRequest): Promise<AccessMapping> {
    return getResponse(
        backend.fetchJson(`${REST_PATH}/${id}/access`, {
            method: 'POST',
            body: JSON.stringify(item),
            headers: { 'Content-Type': 'application/json' }
        }),
        (data) => data as AccessMapping,
        `${REST_PATH}/${id}/access`
    );
}

export async function revokePackageAccess(backend: IBackend, id: string, address: string): Promise<AccessMapping> {
    return getResponse(
        backend.fetchJson(`${REST_PATH}/${id}/access/${encodeURIComponent(address)}`, { method: 'DELETE' }),
        (data) => data as AccessMapping,
        `${REST_PATH}/${id}/access/${address}`
    );
}

export async function myPackageAccess(backend: IBackend, id: string): Promise<AccessRole[]> {
    return getResponse(
        backend.fetchJson(`${REST_PATH}/${id}/my/access`),
        (data) => data as AccessRole[],
        `${REST_PATH}/${id}/my/access`
    );
}

export class PackageApi implements IAccessMapApi, ICrudApi<TPackage, TPackageNew, Partial<Omit<TPackage, 'id'>>>, IAccessApi {
    private _backend: IBackend

    constructor(backend: IBackend) {
        this._backend = backend;
    }

    public get backend() {
        return this._backend;
    }

    public list = query(() => getPackages(this.backend), ENTITIES_NAME)
    public get = query((id: string) => getPackage(this.backend, id), ENTITIES_NAME)
    public create = query((item: TPackageNew) => addPackage(this.backend, item), `add_${ENTITIES_NAME}`)
    public update = query((id: string, item: Partial<Omit<TPackage, 'id'>>) => updatePackage(this.backend, id, item), `update_${ENTITIES_NAME}`)
    public remove = query((id: string) => removePackage(this.backend, id), `remove_${ENTITIES_NAME}`)
    public listAccess = query((id: string) => listPackageAccess(this.backend, id), `${ENTITIES_NAME}_access`)
    public grantAccess = query((id: string, item: GrantRequest) => grantPackageAccess(this.backend, id, item), `${ENTITIES_NAME}_grant`)
    public revokeAccess = query((id: string, address: string) => revokePackageAccess(this.backend, id, address), `${ENTITIES_NAME}_revoke`)
    public myAccess = query((id: string) => myPackageAccess(this.backend, id), `${ENTITIES_NAME}_my_access`)
}

let packageApi: PackageApi | undefined;

export function getPackageApi(backend: IBackend): PackageApi {
    if (!packageApi || packageApi.backend !== backend) {
        packageApi = new PackageApi(backend);
    }
    return packageApi;
}
