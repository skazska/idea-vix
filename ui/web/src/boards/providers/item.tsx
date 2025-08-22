import { createContext, useContext, type ParentComponent } from "solid-js";
import { getBoardApi } from "./api";
import { useBackend } from "../../common/providers/backend";
import { getItemModel, type TItemModel } from "../../common/crud/model";
import type { TBoard, TBoardUpdate } from "../model";

const BoardItemContext = createContext<TItemModel<TBoard, TBoardUpdate>>();

export const BoardItemProvider: ParentComponent<{ boardId: string }> = (props) => {
    const boardApi = getBoardApi(useBackend());
    const model = getItemModel(boardApi, props.boardId);

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
