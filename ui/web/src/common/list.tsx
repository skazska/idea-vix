import type { ParentComponent } from "solid-js";

type ContainerComponent<T = {}> = ParentComponent<{ type?: ListContainerLayoutType, class?: string } & T>;
type ListContainerLayoutType = "row" | "column" | "block";


const ITEM_CONTAINER_STYLES: { [key in ListContainerLayoutType]: string } = {
    row: "flex flex-row py-1",
    column: "flex flex-col px-1",
    block: "block border",
};

export const ListItemContainer: ContainerComponent = (props) => {
    return (
        <div class={`${ITEM_CONTAINER_STYLES[props.type || "row"]} shadow shadow-gray-900 ${props.class}`}>
            {props.children}
        </div>
    );
}

export const ListItemRowContainer: ContainerComponent = (props) => {
    return (
        <div class={`${ITEM_CONTAINER_STYLES[props.type || "column"]} border-r border-gray-300 last:border-r-0 ${props.class}`}>
            {props.children}
        </div>
    );
};

export const ListItemColumnContainer: ContainerComponent = (props) => {
    return (
        <div class={`${ITEM_CONTAINER_STYLES[props.type || "row"]} border-b border-gray-300 last:border-b-0 ${props.class}`}>
            {props.children}
        </div>
    );
}
