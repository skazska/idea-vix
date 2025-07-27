import { createContext, useContext, type ParentComponent } from "solid-js";
import type { TBoard } from "../model";
import { revalidate } from "@solidjs/router";
import { getBoardApi, BoardApi } from "./api";
import { useBackend } from "../../common/providers/backend";
import { createAsync, type AccessorWithLatest } from "@solidjs/router";

export type TBoardItemModel = [
    AccessorWithLatest<TBoard | undefined>,
    {
        remove: BoardApi["removeBoard"];
        update: BoardApi["updateBoard"];
        reload: () => void;
    }
];

const BoardItemContext = createContext<TBoardItemModel>();

export const BoardItemProvider: ParentComponent<{ boardId: string }> = (props) => {
    const boardApi = getBoardApi(useBackend());
    const { updateBoard, removeBoard } = boardApi;
    const resource = createAsync(() => boardApi.getBoard(props.boardId), { 
        name: "board-item-query",
    });

    console.log("BoardItemProvider rendered for board:", props.boardId);

    const model: TBoardItemModel = [
        resource,
        {
            remove: removeBoard.bind(boardApi),
            update: updateBoard.bind(boardApi),
            reload: () => {
                const key = boardApi.getBoard.keyFor(props.boardId);
                console.log("Reloading board item with revalidate key:", key);
                revalidate(key);
            }
        }
    ];

    return (
        <BoardItemContext.Provider value={model}>
            {props.children}
        </BoardItemContext.Provider>
    );
};

export function useBoardItem() {
    if (!BoardItemContext) {
        throw new Error("no context: useBoardItem must be used within a BoardItemProvider");
    }

    const context = useContext(BoardItemContext);

    if (!context) {
        throw new Error("context is empty: useBoardItem must be used within a BoardItemProvider");
    }

    return context;
}
