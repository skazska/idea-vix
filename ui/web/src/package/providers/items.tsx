import { createContext, useContext, type ParentComponent } from "solid-js";
import type { TPackage } from "../model";
import { reload } from "@solidjs/router";
import { getPackageApi, PackageApi } from "./api";
import { useBackend } from "../../common/providers/backend";
import { createAsync, type AccessorWithLatest } from "@solidjs/router";
//import { reconcile } from "solid-js/store";

export type TPackageItemsModel = [
    AccessorWithLatest<TPackage[] | undefined >,
    {
        add: PackageApi["addPackage"];
        remove: PackageApi["removePackage"];
        reload: () => void;
    }
];

const PackageItemsContext = createContext<TPackageItemsModel>();


export const PackageProvider: ParentComponent<{ items: TPackage[] }> = (props) => {
    const packageApi = getPackageApi(useBackend());
    const resource = createAsync(() => packageApi.getPackages(), {  name: "package-query", 
        // reconcile: 
        // reconcile({
        //     deep: true,
        //     // merge: (prev, next) => [...prev, ...next],
        //     // merge: (prev, next) => prev.concat(next),
        //     merge: (prev, next) => {
        //         if (!prev) return next;
        //         if (!next) return prev;
        //         return [...prev, ...next];
        //     }
        // })
    });

    console.log("PackageItemsProvider rendered");

    const model:TPackageItemsModel = [
        resource,
        {
            add: packageApi.addPackage.bind(packageApi),
            remove: packageApi.removePackage.bind(packageApi),
            reload: () => {
                console.log("PackageItemsProvider reload, for key:", packageApi.getPackages.key);
                reload({ revalidate: packageApi.getPackage.key });
                // actions.refetch({ revalidate: packageApi.getPackages.key });
            }
        }
    ];

    return (
        <PackageItemsContext.Provider value={model}>
            {props.children}
        </PackageItemsContext.Provider>
    )
}

export function usePackageData() {
    if (!PackageItemsContext) {
        throw new Error("no context: usePackage must be used within a PackageProvider");
    }

    const context = useContext(PackageItemsContext);

    if (!context) {
        throw new Error("context is empty: usePackage must be used within a PackageProvider");
    }

    return context;
}
