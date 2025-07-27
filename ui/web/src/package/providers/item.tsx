import { createContext, useContext, type ParentComponent } from "solid-js";
import type { TPackage } from "../model";
import { revalidate } from "@solidjs/router";
import { getPackageApi, PackageApi } from "./api";
import { useBackend } from "../../common/providers/backend";
import { createAsync, type AccessorWithLatest } from "@solidjs/router";

export type TPackageItemModel = [
    AccessorWithLatest<TPackage | undefined>,
    {
        remove: PackageApi["removePackage"];
        update: PackageApi["updatePackage"];
        reload: () => void;
    }
];

const PackageItemContext = createContext<TPackageItemModel>();

export const PackageItemProvider: ParentComponent<{ packageId: string }> = (props) => {
    const packageApi = getPackageApi(useBackend());
    const { updatePackage, removePackage } = packageApi;
    const resource = createAsync(() => packageApi.getPackage(props.packageId), { 
        name: "package-item-query",
    });

    console.log("PackageItemProvider rendered for package:", props.packageId);

    const model: TPackageItemModel = [
        resource,
        {
            remove: removePackage.bind(packageApi),
            update: updatePackage.bind(packageApi),
            reload: () => {
                const key = packageApi.getPackage.keyFor(props.packageId);
                console.log("Reloading package item with revalidate key:", key);
                revalidate(key);
            }
        }
    ];

    return (
        <PackageItemContext.Provider value={model}>
            {props.children}
        </PackageItemContext.Provider>
    );
};

export function usePackageItem() {
    if (!PackageItemContext) {
        throw new Error("no context: usePackageItem must be used within a PackageItemProvider");
    }

    const context = useContext(PackageItemContext);

    if (!context) {
        throw new Error("context is empty: usePackageItem must be used within a PackageItemProvider");
    }

    return context;
}
