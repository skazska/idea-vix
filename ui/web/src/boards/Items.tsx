import { Index, type Component } from "solid-js";
import { useBoards } from "./providers/boards";

const BoardsItems: Component = () => {
    const [boards, actions] = useBoards();

    return (
        <>
            <Index each={boards()}>{(item, _index) => {
                return (
                    <div>
                        <h3>{item().name}</h3>
                        <p>{item().description}</p>
                    </div>
                );
            }}</Index>
            <button onClick={() => actions.add()}>Add to Cart</button>
        </>
    )
}

export default BoardsItems;
