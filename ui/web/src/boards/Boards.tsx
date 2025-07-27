import { createSignal, ErrorBoundary, Show } from "solid-js";
import { setTitle } from "../common/providers/page-state";
import { BoardsProvider } from "./providers/items";
import BoardsItems from "./Items";
import { Portal } from "solid-js/web";
import { Plus } from "lucide-solid";
import { NewBoardForm } from "./forms/new";
import { ModalCentered } from "../common/modals";
import type { TBoardNew } from "./model";
import { ENTITIES_NAME, PAGE_TITLE } from "./const";

export default function Boards() {
    setTitle(PAGE_TITLE)
    const [formNew, showFormNew] = createSignal(false)

    const add = (values: TBoardNew | undefined) => {
        if (values) showFormNew(false)
    }

    const cancelAdd = () => showFormNew(false)

    console.log("Boards rendered");

    return (
        <div class="p-4">
            <Portal mount={document.getElementById("sub-menu")!}>
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-m"
                        onClick={() => showFormNew(true)}>
                    <Plus/>
                </button>
            </Portal>
            <div>
                <ErrorBoundary fallback={(error) => (
                    <div class="error">
                        <h2>Error loading items</h2>
                        <p>{error.message}</p>
                    </div>
                )}>
                    <BoardsProvider items={[]}>
                        <Show when={formNew()}>
                            <Portal mount={document.querySelector('main')!}>
                                <ModalCentered>
                                    <NewBoardForm onCancel={cancelAdd} onDone={add} />
                                </ModalCentered>
                            </Portal>
                        </Show>
                        <BoardsItems />
                    </BoardsProvider>
                </ErrorBoundary>
            </div>
        </div>
    );
}
