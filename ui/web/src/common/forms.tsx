import { mergeProps, Show, type Accessor, type ParentComponent } from "solid-js";
import { Spin } from "./statics";

// exports submit form button
export const Submit: ParentComponent = (props) =>(
    <button type="submit" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
        {props.children || "Submit"}
    </button>
)

// exports reset form button
export const Reset: ParentComponent = (props) => (
    <button type="reset" class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded">
        {props.children || "Reset"}
    </button>
);

// exports close form button
export const Close: ParentComponent<{ onClick: () => void }> = (props) => (
    <button type="button" class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" onClick={props.onClick}>
        {props.children || "Close"}
    </button>
);

export const Header: ParentComponent<{ text: Accessor<string | undefined>; pending?: Accessor<boolean | undefined> }> = (props) => {
    const myProps = mergeProps(props);
    return (
    <div class="flex items-center justify-between pb-4 md:pb-5 border-b rounded-t dark:border-gray-600 border-gray-200">
        <h2 class="text-lg font-semibold">{myProps.text()}</h2>
        {myProps.children}
        <Show when={myProps.pending?.()}>
            <Spin />
        </Show>
    </div>
)}

export const Error: ParentComponent<{ error: Accessor<string | undefined> }> = (props) => {

    return (
        <div class="text-red-500 px-2 h-10 overflow-hidden hover:overflow-auto hover:h-30">
            {props.error() ? props.error() : "An error occurred"}
        </div>
    );
};

export const Footer: ParentComponent<{ onCancel: () => void, error: Accessor<string | undefined> }> = (props) => (
    <div class="flex items-center justify-between pt-4 md:pt-5 border-t rounded-b dark:border-gray-600 border-gray-200">
        <Close onClick={props.onCancel}>Close</Close>
        <Show when={props.error()}>
            <Error error={props.error} />
        </Show>
        <div class="flex items-center space-x-2">
            {props.children}
            <Submit />
        </div>        
    </div>
)
