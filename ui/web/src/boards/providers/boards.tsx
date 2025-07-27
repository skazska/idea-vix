import { createContext, createResource, useContext, type ParentComponent, type Resource } from "solid-js";
import { boardsApi } from "../api";
import type { TBoard } from "../types";

export type TBoardsModel = [
    Resource<TBoard[]>,
    {
        reload: () => void;
    }
];


const BoardsContext = createContext<TBoardsModel>();

export const BoardsProvider: ParentComponent<{}> = (props) => {
    const [resource, actions] = createResource(async () => boardsApi.getBoards(), {  name: "boards-query" })

    const model: TBoardsModel = [
        resource,
        {
            reload: () => {
                actions.refetch();
            }
        }
    ];

    return (
        <BoardsContext.Provider value={model}>
            {props.children}
        </BoardsContext.Provider>
    );
};

export function useBoards() {
    if (!BoardsContext) {
        throw new Error("useBoards must be used within a BoardsProvider");
    }

    const context = useContext(BoardsContext);

    if (!context) {
        throw new Error("useBoards must be used within a BoardsProvider");
    }

    return context;
}