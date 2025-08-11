import type { JSXElement, JSX, Component } from "solid-js";

// form field state interface
export type IFieldState<V> = {
    error?: string; // error message
    value?: V; // field value
}

// form field props interface
export type IFieldProps = {
    name: string; // field name
    label?: string; // field label
    placeholder?: string; // field placeholder
    required?: boolean; // is field required
}

// form field element props interface
export type IFieldElementProps<E extends HTMLElement> = {
    ref: (element: E) => void; // ref to the input element
    onInput: JSX.EventHandler<E, InputEvent>; // input event handler
    onChange: JSX.EventHandler<E, Event>; // change event handler
    onBlur: JSX.EventHandler<E, FocusEvent>; // blur event handler
}

// form field error element getter type
export type TFieldErrorGetter = (state: { error?: string }) => JSXElement;
// form field label element getter type
export type TFieldLabelGetter = (field: IFieldProps) => JSXElement;
// form field string editor element getter type
export type TFieldStringEditorGetter = (field: IFieldProps, state: IFieldState<string>, props:IFieldElementProps<HTMLInputElement>) => JSXElement;
// form field text editor element getter type
export type TFieldTextEditorGetter = (field: IFieldProps, state: IFieldState<string>, props:IFieldElementProps<HTMLTextAreaElement>) => JSXElement;
// form field number editor element getter type
export type TFieldNumberEditorGetter = (field: IFieldProps, state: IFieldState<number>, props:IFieldElementProps<HTMLInputElement>) => JSXElement;
// form field boolean editor element getter type
export type TFieldBooleanEditorGetter = (field: IFieldProps, state: IFieldState<boolean>, props:IFieldElementProps<HTMLInputElement>) => JSXElement;


// form Component type
export type TFormComponent<I, R> = Component<{
    onDone: (values: R | undefined) => void;
    onCancel: () => void;
    initialValues?: I;
}>;



// returns form input's error message if it exists
export const getError: TFieldErrorGetter = (state) => state.error ? (<div class="text-red-500">{state.error}</div>) : null


// returns a label for the form input
export const getLabel: TFieldLabelGetter = (field) => (
    <label class="block text-gray-700 text-sm font-bold mb-2" for={field.name}>
        {field.label || field.name.charAt(0).toUpperCase() + field.name.slice(1)}
        {field.required ? <span class="text-red-500">*</span> : null}
    </label>
)

// returns a text input
export const getString: TFieldStringEditorGetter = (field, state, props): JSXElement => {
    return <input {...props} type="text" 
        class="border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" 
        placeholder={field.placeholder}
        value={state.value}
    />;
}

// returns a textarea input
export const getText: TFieldTextEditorGetter = (field, state, props): JSXElement => {
    return <textarea {...props}
        class="border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" 
        placeholder={field.placeholder}
        value={state.value}
    />; 
}

// returns a number input
export const getNumber: TFieldNumberEditorGetter = (field, state, props): JSXElement => {
    return <input {...props} type="number"
        class="border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" 
        placeholder={field.placeholder}
        value={state.value || 0} // default to 0 if no value
    />;
}

// returns a checkbox input for boolean
export const getBoolean: TFieldBooleanEditorGetter = (field, state, props): JSXElement => {
    return (
        <label class="inline-flex items-center gap-2 select-none">
            <input {...props} type="checkbox"
                class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                checked={!!state.value}
            />
            <span class="text-gray-700">{field.label || field.name.charAt(0).toUpperCase() + field.name.slice(1)}</span>
        </label>
    );
}

// returns field contents
export const getInput = (...args: JSXElement[]): JSXElement => (<>{...args}</>)
