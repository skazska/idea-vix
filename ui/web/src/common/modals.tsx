import type { ParentComponent } from "solid-js";

export const ModalCentered: ParentComponent<{}> = (props) => {
    return (<div aria-hidden="true" class="bg-gray-500/30 overflow-y-auto overflow-x-hidden absolute top-0 right-0 left-0 z-100 flex flex-row justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full">
        <div role="dialog" class="bg-white text-gray-900 relative p-4 w-full max-w-md max-h-full z-101 shadow-gray-500 shadow-2xl bg-blend-normal" data-testid="modal-centered">
            {props.children}
        </div>
    </div>)
}
