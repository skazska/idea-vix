import { query } from "@solidjs/router";
import type { TBoard } from "./types";
import { getResponse, useBackend } from "../common/providers/backend";

export async function getBoards(): Promise<TBoard[]> {
    return getResponse(useBackend().fetchJson("/api/boards"), (data) => data as TBoard[], '/api/boards');
}

export async function getBoard(id: string): Promise<TBoard> {
    return getResponse(useBackend().fetchJson("/api/boards/${id}"), (data) => data as TBoard, `/api/boards/${id}`);
}

export const boardsApi = {
    getBoards: query(async () => await getBoards(), "boards"),
    getBoard: query(async (id: string) => await getBoard(id), "board"),
}
