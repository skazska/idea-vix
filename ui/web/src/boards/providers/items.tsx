import { createContext, useContext, type ParentComponent } from "solid-js";
import type { TBoard, TBoardNew } from "../model";
import { getBoardApi } from "./api";
import { useBackend } from "../../common/providers/backend";
import { getItemsModel, type TItemsModel } from "../../common/crud/model";

export type TBoardItemsModel = TItemsModel<TBoard, TBoardNew>;

const BoardItemsContext = createContext<TBoardItemsModel>();

export const BoardsProvider: ParentComponent<{ items?: TBoard[] }> = (props) => {
    const boardApi = getBoardApi(useBackend());

    const model: TBoardItemsModel = getItemsModel(boardApi);

    return (
        <BoardItemsContext.Provider value={model}>
            {props.children}
        </BoardItemsContext.Provider>
    );
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
