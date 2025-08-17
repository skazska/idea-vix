/* @refresh reload */

import { query } from "@solidjs/router";
import type { TPackage, TPackageNew } from "../model";
import type { AccessMapping, GrantRequest } from "../../common/access/model";
import { getResponse, type IBackend } from "../../common/providers/backend";
import { REST_PATH } from "../const";

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

export class PackageApi {
    private _backend: IBackend

    constructor(backend: IBackend) {
        this._backend = backend;
    }

    public get backend() {
        return this._backend;
    }

    public getPackages = query(() => getPackages(this.backend), "package")
    public getPackage = query((id: string) => getPackage(this.backend, id), "package")
    public addPackage = query((item: TPackageNew) => addPackage(this.backend, item), "addPackage")
    public updatePackage = query((id: string, item: Partial<Omit<TPackage, 'id'>>) => updatePackage(this.backend, id, item), "updatePackage")
    public removePackage = query((id: string) => removePackage(this.backend,id), "removePackage")
    public listAccess = (id: string) => listPackageAccess(this.backend, id)
    public grantAccess = (id: string, item: GrantRequest) => grantPackageAccess(this.backend, id, item)
    public revokeAccess = (id: string, address: string) => revokePackageAccess(this.backend, id, address)
}

let packageApi: PackageApi | undefined;

export function getPackageApi(backend: IBackend): PackageApi {
    if (!packageApi || packageApi.backend !== backend) {
        packageApi = new PackageApi(backend);
    }
    return packageApi;
}
