import { createContext, useContext, type ParentComponent } from "solid-js";
import { getAccessModel, type IAccessApi, type TAccessModel } from "./model";

const AccessContext = createContext<TAccessModel>();

export const AccessProvider: ParentComponent<{ id: string, api: IAccessApi }> = (props) => {
    const model: TAccessModel = getAccessModel(props.api, props.id);

    return (
        <AccessContext.Provider value={model}>
            {props.children}
        </AccessContext.Provider>
    );
};

export function useAccess() {
    if (!AccessContext) {
        throw new Error("no context: useAccess must be used within an AccessProvider");
    }

    const context = useContext(AccessContext);

    if (!context) {
        throw new Error("context is empty: useAccess must be used within an AccessProvider");
    }

    return context;
}
