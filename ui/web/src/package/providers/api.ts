/* @refresh reload */

import { query } from "@solidjs/router";
import type { TPackage, TPackageNew } from "../model";
import { getResponse, type IBackend } from "../../common/providers/backend";

export async function getPackages(backend: IBackend): Promise<TPackage[]> {
    return getResponse(backend.fetchJson('/api/package'), (data) => data as TPackage[], '/api/package');
}

export async function getPackage(backend: IBackend, id: string): Promise<TPackage> {
    return getResponse(backend.fetchJson(`/api/package/${id}`), (data) => data as TPackage, `/api/package/${id}`);
}

export async function addPackage(backend: IBackend, item: TPackageNew): Promise<TPackage> {
    return getResponse(backend.fetchJson('/api/package', {
        method: 'POST',
        body: JSON.stringify(item),
        headers: { 'Content-Type': 'application/json' }
    }), (data) => data as TPackage, '/api/package');
}

export async function updatePackage(backend: IBackend, id: string, item: Partial<Omit<TPackage, 'id'>>): Promise<TPackage> {
    return getResponse(backend.fetchJson(`/api/package/${id}`, {
        method: 'PUT',
        body: JSON.stringify(item),
        headers: { 'Content-Type': 'application/json' }
    }), (data) => data as TPackage, `/api/package/${id}`);
}

export async function removePackage(backend: IBackend, id: string): Promise<boolean> {
    return getResponse(backend.fetchJson(`/api/package/${id}`, { method: 'DELETE' }), (data) => data as boolean, `/api/package/${id}`);
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
}

let packageApi: PackageApi | undefined;

export function getPackageApi(backend: IBackend): PackageApi {
    if (!packageApi || packageApi.backend !== backend) {
        packageApi = new PackageApi(backend);
    }
    return packageApi;
}
