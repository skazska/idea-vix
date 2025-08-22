import { createContext, useContext, type ParentComponent } from "solid-js";
import { getAccessMapModel, type IAccessMapApi, type TAccessMapModel } from "./model";

const AccessMapContext = createContext<TAccessMapModel>();

export const AccessMapProvider: ParentComponent<{ id: string, api: IAccessMapApi }> = (props) => {
    const model: TAccessMapModel = getAccessMapModel(props.api, props.id);
    return (
        <AccessMapContext.Provider value={model}>
            {props.children}
        </AccessMapContext.Provider>
    );
};

export function useAccessMap() {
    if (!AccessMapContext) {
        throw new Error("no context: useAccessMap must be used within an AccessMapProvider");
    }

    const context = useContext(AccessMapContext);

    if (!context) {
        throw new Error("context is empty: useAccessMap must be used within an AccessMapProvider");
    }

    return context;
}
