import { createAsync, type RoutePreloadFuncArgs } from "@solidjs/router";
import type { Component } from "solid-js";
import { boardsApi } from "./api";

const Board: Component<RoutePreloadFuncArgs> = (props) => {
    const board = createAsync(() => boardsApi.getBoard(props.params.id));

    return (
        <div class="board">
            <h3>{props.name}</h3>
            {props.description && <p>{props.description}</p>}
            {props.icon && <img src={props.icon} alt={`${props.name} icon`} />}
        </div>
    );
}

export default Board;
