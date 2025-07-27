import { createContext, useContext, type ParentComponent } from "solid-js";
import type { TBoard } from "../model";
import { reload } from "@solidjs/router";
import { getBoardApi, BoardApi } from "./api";
import { useBackend } from "../../common/providers/backend";
import { createAsync, type AccessorWithLatest } from "@solidjs/router";

export type TBoardItemsModel = [
    AccessorWithLatest<TBoard[] | undefined >,
    {
        add: BoardApi["addBoard"];
        remove: BoardApi["removeBoard"];
        reload: () => void;
    }
];

const BoardItemsContext = createContext<TBoardItemsModel>();


export const BoardsProvider: ParentComponent<{ items?: TBoard[] }> = (props) => {
    const boardApi = getBoardApi(useBackend());
    const resource = createAsync(() => boardApi.getBoards(), {  name: "boards-query" });

    console.log("BoardItemsProvider rendered");

    const model: TBoardItemsModel = [
        resource,
        {
            add: boardApi.addBoard.bind(boardApi),
            remove: boardApi.removeBoard.bind(boardApi),
            reload: () => {
                console.log("BoardItemsProvider reload, for key:", boardApi.getBoards.key);
                reload({ revalidate: boardApi.getBoards.key });
            }
        }
    ];

    return (
        <BoardItemsContext.Provider value={model}>
            {props.children}
        </BoardItemsContext.Provider>
    )
}

export function useBoardsData() {
    if (!BoardItemsContext) {
        throw new Error("no context: useBoards must be used within a BoardsProvider");
    }

    const context = useContext(BoardItemsContext);

    if (!context) {
        throw new Error("context is empty: useBoards must be used within a BoardsProvider");
    }

    return context;
}
