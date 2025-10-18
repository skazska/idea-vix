import { createSignal, Show, For, createResource } from "solid-js";
import { Package as PackageIcon, Plus, Trash2, Search } from "lucide-solid";
import { Portal } from "solid-js/web";
import { revalidate } from "@solidjs/router";
import { ModalCentered } from "../common/modals";
import { getBoardApi } from "./providers/api";
import { listBoardPackages } from "./providers/api";
import { getPackageApi } from "../package/providers/api";
import { useBackend } from "../common/providers/backend";

interface BoardPackagesProps {
    boardId: string;
}

export default function BoardPackages(props: BoardPackagesProps) {
    const backend = useBackend();
    const boardApi = getBoardApi(backend);
    const packageApi = getPackageApi(backend);

    const [showAddModal, setShowAddModal] = createSignal(false);
    const [searchTerm, setSearchTerm] = createSignal("");
    const [adding, setAdding] = createSignal(false);
    const [removing, setRemoving] = createSignal<number | null>(null);

    // Load board packages
    const [boardPackages, { refetch: refetchBoardPackages }] = createResource(
        () => props.boardId,
        (id) => listBoardPackages(backend, id)
    );

    // Load all available packages
    const [availablePackages] = createResource(() => packageApi.list());

    const handleAddPackage = async (packageId: number) => {
        setAdding(true);
        try {
            await boardApi.addPackage(props.boardId, packageId);
            // Force immediate refetch and wait for completion
            await refetchBoardPackages();
            revalidate(boardApi.listPackages.key);
            setShowAddModal(false);
            setSearchTerm("");
        } catch (error) {
            console.error("Failed to add package:", error);
            alert("Failed to add package. It may already be added or you don't have permission.");
        } finally {
            setAdding(false);
        }
    };

    const handleRemovePackage = async (packageId: number) => {
        if (!confirm("Are you sure you want to remove this package from the board?")) {
            return;
        }

        setRemoving(packageId);
        try {
            await boardApi.removePackage(props.boardId, packageId);
            // Force immediate refetch and wait for completion
            await refetchBoardPackages();
            revalidate(boardApi.listPackages.key);
        } catch (error) {
            console.error("Failed to remove package:", error);
            alert("Failed to remove package.");
        } finally {
            setRemoving(null);
        }
    };

    const filteredAvailablePackages = () => {
        const available = availablePackages() || [];
        const boardPkgs = boardPackages() || [];
        const boardPkgIds = new Set(boardPkgs.map(p => Number(p.id)));
        const term = searchTerm().toLowerCase();

        return available
            .filter(pkg => !boardPkgIds.has(Number(pkg.id)))
            .filter(pkg => 
                term === "" || 
                pkg.name.toLowerCase().includes(term) ||
                pkg.slug.toLowerCase().includes(term) ||
                (pkg.description || "").toLowerCase().includes(term)
            );
    };

    return (
        <div class="bg-white rounded-lg shadow p-6">
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-semibold flex items-center gap-2">
                    <PackageIcon size={20} />
                    Packages
                </h3>
                <button
                    data-testid="add-package-button"
                    class="flex items-center gap-2 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors"
                    onClick={() => setShowAddModal(true)}
                    disabled={adding()}
                >
                    <Plus size={16} />
                    Add Package
                </button>
            </div>

            <Show
                when={!boardPackages.loading}
                fallback={<div class="text-gray-500">Loading packages...</div>}
            >
                <Show
                    when={(boardPackages() || []).length > 0}
                    fallback={
                        <div class="text-gray-500 text-center py-8">
                            No packages added yet. Click "Add Package" to get started.
                        </div>
                    }
                >
                    <div class="space-y-2">
                        <For each={boardPackages()}>
                            {(pkg) => (
                                <div
                                    data-testid={`board-package-${pkg.slug}`}
                                    class="flex items-center justify-between p-3 border border-gray-200 rounded hover:bg-gray-50 transition-colors"
                                >
                                    <div class="flex items-center gap-3">
                                        <Show when={pkg.icon}>
                                            <img src={pkg.icon} alt={pkg.name} class="w-8 h-8 rounded" />
                                        </Show>
                                        <div>
                                            <div class="font-semibold">{pkg.name}</div>
                                            <div class="text-sm text-gray-600">{pkg.slug}</div>
                                            <Show when={pkg.description}>
                                                <div class="text-xs text-gray-500 mt-1">{pkg.description}</div>
                                            </Show>
                                        </div>
                                    </div>
                                    <button
                                        data-testid={`remove-package-${pkg.slug}`}
                                        class="text-red-500 hover:text-red-700 p-2 rounded hover:bg-red-50 transition-colors"
                                        onClick={() => handleRemovePackage(Number(pkg.id))}
                                        disabled={removing() === Number(pkg.id)}
                                        title="Remove package"
                                    >
                                        <Show
                                            when={removing() !== Number(pkg.id)}
                                            fallback={<span class="text-xs">Removing...</span>}
                                        >
                                            <Trash2 size={16} />
                                        </Show>
                                    </button>
                                </div>
                            )}
                        </For>
                    </div>
                </Show>
            </Show>

            {/* Add Package Modal */}
            <Show when={showAddModal()}>
                <Portal mount={document.querySelector('main')!}>
                    <ModalCentered>
                        <div class="p-6 max-w-2xl w-full max-h-[80vh] flex flex-col">
                            <h3 class="text-xl font-bold mb-4">Add Package to Board</h3>

                            {/* Search */}
                            <div class="mb-4">
                                <div class="relative">
                                    <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={20} />
                                    <input
                                        data-testid="package-search-input"
                                        type="text"
                                        class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                        placeholder="Search packages..."
                                        value={searchTerm()}
                                        onInput={(e) => setSearchTerm(e.target.value)}
                                    />
                                </div>
                            </div>

                            {/* Package List */}
                            <div class="flex-1 overflow-y-auto mb-4">
                                <Show
                                    when={!availablePackages.loading}
                                    fallback={<div class="text-gray-500 text-center py-4">Loading available packages...</div>}
                                >
                                    <Show
                                        when={filteredAvailablePackages().length > 0}
                                        fallback={
                                            <div class="text-gray-500 text-center py-8">
                                                <Show
                                                    when={searchTerm() === ""}
                                                    fallback={<p>No packages found matching "{searchTerm()}"</p>}
                                                >
                                                    <p>All available packages have been added to this board.</p>
                                                </Show>
                                            </div>
                                        }
                                    >
                                        <div class="space-y-2">
                                            <For each={filteredAvailablePackages()}>
                                                {(pkg) => (
                                                    <div
                                                        data-testid={`available-package-${pkg.slug}`}
                                                        class="flex items-center justify-between p-3 border border-gray-200 rounded hover:bg-gray-50 transition-colors"
                                                    >
                                                        <div class="flex items-center gap-3 flex-1">
                                                            <Show when={pkg.icon}>
                                                                <img src={pkg.icon} alt={pkg.name} class="w-8 h-8 rounded" />
                                                            </Show>
                                                            <div class="flex-1">
                                                                <div class="font-semibold flex items-center gap-2">
                                                                    {pkg.name}
                                                                    <span class={`text-xs px-2 py-0.5 rounded ${pkg.is_public ? 'bg-green-100 text-green-700' : 'bg-gray-200 text-gray-700'}`}>
                                                                        {pkg.is_public ? 'Public' : 'Private'}
                                                                    </span>
                                                                </div>
                                                                <div class="text-sm text-gray-600">{pkg.slug}</div>
                                                                <Show when={pkg.description}>
                                                                    <div class="text-xs text-gray-500 mt-1">{pkg.description}</div>
                                                                </Show>
                                                            </div>
                                                        </div>
                                                        <button
                                                            data-testid={`add-package-to-board-${pkg.slug}`}
                                                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors whitespace-nowrap"
                                                            onClick={() => handleAddPackage(Number(pkg.id))}
                                                            disabled={adding()}
                                                        >
                                                            Add
                                                        </button>
                                                    </div>
                                                )}
                                            </For>
                                        </div>
                                    </Show>
                                </Show>
                            </div>

                            {/* Close Button */}
                            <div class="flex justify-end">
                                <button
                                    data-testid="close-modal-button"
                                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition-colors"
                                    onClick={() => {
                                        setShowAddModal(false);
                                        setSearchTerm("");
                                    }}
                                    disabled={adding()}
                                >
                                    Close
                                </button>
                            </div>
                        </div>
                    </ModalCentered>
                </Portal>
            </Show>
        </div>
    );
}
