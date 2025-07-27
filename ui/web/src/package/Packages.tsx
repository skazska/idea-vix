// Solidjs typescript App for the package page

import { createEffect, createSignal, ErrorBoundary, Show } from "solid-js";
import { setTitle } from "../common/providers/page-state";
import PackageItems from "./Items";
import { PackageProvider } from "./providers/items";
import { Portal } from "solid-js/web";
import { PackagePlus } from "lucide-solid";
import { NewPackageForm } from "./forms/new";
import { ModalCentered } from "../common/modals";
import type { TPackageNew } from "./model";

export default function Packages() {
    setTitle("Packages");
    const [formNew, showFormNew] = createSignal(false);

    const add = (values: TPackageNew | undefined) => {
        console.log("Addedding new package item", values);
        if (values) { 
            console.log("Added new package item", values);
            showFormNew(false)
        }    
    };

    createEffect(() => {
        console.log("Form new state changed:", formNew());
    });

    const cancelAdd = () => showFormNew(false)

    console.log("Package rendered");

    return (
        <div class="p-4">
            <Portal mount={document.getElementById("sub-menu")!}>
            
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-m"
                        onClick={() => showFormNew(true)}>
                    <PackagePlus/>
                </button>
            </Portal>
            <h2>Welcome to the Package Page</h2>
            <div>
            <ErrorBoundary fallback={(error) => (
                <div class="error">
                    <h2>Error loading package items</h2>
                    <p>{error.message}</p>
                </div>
            )}>
                <PackageProvider items={[]}>
                    <Show when={formNew()}>
                        <Portal mount={document.querySelector('main')!}>
                            <ModalCentered>
                                <NewPackageForm onCancel={cancelAdd} onDone={add} />
                            </ModalCentered>
                        </Portal>
                    </Show>
                    <PackageItems />
                </PackageProvider>
            </ErrorBoundary>
            </div>

        </div>
    );
}