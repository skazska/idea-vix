import { query } from "@solidjs/router";
import type { TBoard, TBoardNew } from "../model";
import { getResponse, type IBackend } from "../../common/providers/backend";
import { ENTITIES_NAME, REST_PATH } from "../const";

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

export class BoardApi {
    private _backend: IBackend

    constructor(backend: IBackend) {
        this._backend = backend;
    }

    public get backend() {
        return this._backend;
    }

    public getBoards = query(() => getBoards(this.backend), ENTITIES_NAME)
    public getBoard = query((id: string) => getBoard(this.backend, id), ENTITIES_NAME)
    public addBoard = query((item: TBoardNew) => addBoard(this.backend, item), `add_${ENTITIES_NAME}`)
    public updateBoard = query((id: string, item: Partial<Omit<TBoard, 'id'>>) => updateBoard(this.backend, id, item), `update_${ENTITIES_NAME}`)
    public removeBoard = query((id: string) => removeBoard(this.backend, id), `remove_${ENTITIES_NAME}`)
}

let boardApi: BoardApi | undefined;

export function getBoardApi(backend: IBackend): BoardApi {
    if (!boardApi || boardApi.backend !== backend) {
        boardApi = new BoardApi(backend);
    }
    return boardApi;
}
