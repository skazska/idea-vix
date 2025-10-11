import { query } from "@solidjs/router";
import type { TBoard, TBoardNew, TBoardUpdate } from "../model";
import type { AccessMapping, AccessRole, GrantRequest, IAccessApi, IAccessMapApi } from "../../common/access/model";
import { getResponse, type IBackend } from "../../common/providers/backend";
import { ENTITIES_NAME, REST_PATH } from "../const";
import type { ICrudApi } from "../../common/crud/model";

export async function getBoards(backend: IBackend): Promise<TBoard[]> {
    return getResponse(backend.fetchJson(REST_PATH), (data) => data as TBoard[], REST_PATH);
}

export async function getBoard(backend: IBackend, id: string): Promise<TBoard> {
    return getResponse(backend.fetchJson(`${REST_PATH}/${id}`), (data) => data as TBoard, `${REST_PATH}/${id}`);
}

export async function addBoard(backend: IBackend, item: TBoardNew): Promise<TBoard> {
    return getResponse(backend.fetchJson(REST_PATH, {
        method: 'POST',
        body: JSON.stringify(item),
        headers: { 'Content-Type': 'application/json' }
    }), (data) => data as TBoard, REST_PATH);
}

export async function updateBoard(backend: IBackend, id: string, item: Partial<Omit<TBoard, 'id'>>): Promise<TBoard> {
    return getResponse(backend.fetchJson(`${REST_PATH}/${id}`, {
        method: 'PUT',
        body: JSON.stringify(item),
        headers: { 'Content-Type': 'application/json' }
    }), (data) => data as TBoard, `${REST_PATH}/${id}`);
}

export async function removeBoard(backend: IBackend, id: string): Promise<boolean> {
    return getResponse(backend.fetchJson(`${REST_PATH}/${id}`, { method: 'DELETE' }), (data) => data as boolean, `${REST_PATH}/${id}`);
}

// Access endpoints
export async function listBoardAccess(backend: IBackend, id: string): Promise<AccessMapping[]> {
    return getResponse(
        backend.fetchJson(`${REST_PATH}/${id}/access`),
        (data) => data as AccessMapping[],
        `${REST_PATH}/${id}/access`
    );
}

export async function grantBoardAccess(backend: IBackend, id: string, item: GrantRequest): Promise<AccessMapping> {
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

export async function revokeBoardAccess(backend: IBackend, id: string, address: string): Promise<AccessMapping> {
    return getResponse(
        backend.fetchJson(`${REST_PATH}/${id}/access/${encodeURIComponent(address)}`, { method: 'DELETE' }),
        (data) => data as AccessMapping,
        `${REST_PATH}/${id}/access/${address}`
    );
}

export async function myBoardAccess(backend: IBackend, id: string): Promise<AccessRole[]> {
    return getResponse(
        backend.fetchJson(`${REST_PATH}/${id}/access/my`),
        (data) => data as AccessRole[],
        `${REST_PATH}/${id}/access/my`
    );
}

export class BoardApi implements IAccessMapApi, ICrudApi<TBoard, TBoardNew, TBoardUpdate>, IAccessApi {
    private _backend: IBackend

    constructor(backend: IBackend) {
        this._backend = backend;
    }

    public get backend() {
        return this._backend;
    }

    public list = query(() => getBoards(this.backend), ENTITIES_NAME)
    public get = query((id: string) => getBoard(this.backend, id), ENTITIES_NAME)
    public create = query((item: TBoardNew) => addBoard(this.backend, item), `add_${ENTITIES_NAME}`)
    public update = query((id: string, item: TBoardUpdate) => updateBoard(this.backend, id, item), `update_${ENTITIES_NAME}`)
    public remove = query((id: string) => removeBoard(this.backend, id), `remove_${ENTITIES_NAME}`)
    public listAccess = query((id: string) => listBoardAccess(this.backend, id), `${ENTITIES_NAME}_access`)
    public grantAccess = query((id: string, item: GrantRequest) => grantBoardAccess(this.backend, id, item), `${ENTITIES_NAME}_grant`)
    public revokeAccess = query((id: string, address: string) => revokeBoardAccess(this.backend, id, address), `${ENTITIES_NAME}_revoke`)
    public myAccess = query((id: string) => myBoardAccess(this.backend, id), `${ENTITIES_NAME}_my_access`)
}

let boardApi: BoardApi | undefined;

export function getBoardApi(backend: IBackend): BoardApi {
    if (!boardApi || boardApi.backend !== backend) {
        boardApi = new BoardApi(backend);
    }
    return boardApi;
}
