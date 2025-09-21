// Solidjs typescript App for the package page

import { createEffect, createSignal, ErrorBoundary, Show } from "solid-js";
import { setTitle } from "../common/providers/page-state";
import PackageItems from "./Items";
import { PackageProvider } from "./providers/items";
import { Portal } from "solid-js/web";
import { Plus } from "lucide-solid";
import { NewPackageForm } from "./forms/new";
import { ModalCentered } from "../common/modals";
import type { TPackageNew } from "./model";
import { PAGE_TITLE } from "./const";

export default function Packages() {
    setTitle(PAGE_TITLE);
    const [formNew, showFormNew] = createSignal(false)

    const add = (values: TPackageNew | undefined) => {
        if (values) showFormNew(false)
    }

    createEffect(() => {
        console.log("Form new state changed:", formNew())
    });

    const cancelAdd = () => showFormNew(false)

    console.log("Package rendered")

    return (
        <div class="p-4">
            <Portal mount={document.getElementById("sub-menu")!}>
            
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-m"
                        onClick={() => showFormNew(true)} data-testid="package-add-button">
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
                <PackageProvider items={[]}>
                    <Show when={formNew()}>
                        <Portal mount={document.querySelector('main')!}>
                            <ModalCentered>
                                <NewPackageForm onCancel={cancelAdd} onDone={add} initialValues={{ name: '', slug: undefined }} />
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