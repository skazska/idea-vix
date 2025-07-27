import { useParams, A, useNavigate } from "@solidjs/router";
import { createSignal, Show, ErrorBoundary, Suspense, createEffect } from "solid-js";
import { setTitle } from "../common/providers/page-state";
import { PackageItemProvider, usePackageItem } from "./providers/item";
import { Portal } from "solid-js/web";
import { Edit, Trash2, ArrowLeft, Plus, Save, X } from "lucide-solid";
import { ModalCentered } from "../common/modals";
import type { TPackage } from "./model";

function PackageContent() {
    console.log("PackageContent rendered");
    const navigate = useNavigate();
    const params = useParams();
    const packageId = params.id;
    
    const [pkg, actions] = usePackageItem();
    const [editMode, setEditMode] = createSignal(false);
    const [showDeleteConfirm, setShowDeleteConfirm] = createSignal(false);
    const [saving, setSaving] = createSignal(false);
    
    // Edit form fields
    const [editName, setEditName] = createSignal("");
    const [editDescription, setEditDescription] = createSignal("");
    const [editIcon, setEditIcon] = createSignal("");
    
    const handleEdit = () => {
        // Initialize edit form with current values
        const currentPkg = pkg.latest;
        if (currentPkg) {
            setEditName(currentPkg.name);
            setEditDescription(currentPkg.description || "");
            setEditIcon(currentPkg.icon || "");
        }
        setEditMode(true);
    };
    
    const handleSave = async () => {
        if (!packageId) return;
        
        setSaving(true);
        try {
            const updates: Partial<Omit<TPackage, 'id'>> = {
                name: editName().trim(),
                description: editDescription().trim(),
                icon: editIcon().trim() || undefined,
            };
            
            await actions.update(packageId, updates);
            setEditMode(false);
            // Reload to get fresh data
            actions.reload();
        } catch (error) {
            console.error('Failed to save package:', error);
        } finally {
            setSaving(false);
        }
    };
    
    const handleCancelEdit = () => {
        setEditMode(false);
        // Reset form fields
        setEditName("");
        setEditDescription("");
        setEditIcon("");
    };
    
    const handleDelete = async () => {
        if (!packageId) return;
        try {
            await actions.remove(packageId);
            // Navigate back to packages list using SolidJS router
            navigate('/packages');
        } catch (error) {
            console.error('Failed to delete package:', error);
        }
    };
    
    const confirmDelete = () => {
        setShowDeleteConfirm(true);
    };
    
    const cancelDelete = () => {
        setShowDeleteConfirm(false);
    };

    createEffect(() => {
        setTitle(`Packages ${pkg.latest?.name}`);
    });

    return (
        <>
            {/* Navigation and action buttons in header */}
            <Portal mount={document.getElementById("sub-menu")!}>
                <div class="flex gap-2">
                    <A href="/packages" class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-1 px-2 rounded text-m">
                        <ArrowLeft size={'1rem'}/>
                    </A>
                    <Show when={pkg.latest}>
                        <Show when={!editMode()} fallback={
                            <>
                                <button 
                                    class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-m"
                                    onClick={handleSave}
                                    disabled={saving()}
                                >
                                    <Save size={'1rem'}/>
                                </button>
                                <button 
                                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-1 px-2 rounded text-m"
                                    onClick={handleCancelEdit}
                                    disabled={saving()}
                                >
                                    <X size={'1rem'}/>
                                </button>
                            </>
                        }>
                            <button 
                                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-m"
                                onClick={handleEdit}
                            >
                                <Edit size={'1rem'}/>
                            </button>
                            <button 
                                class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-m"
                                onClick={confirmDelete}
                            >
                                <Trash2 size={'1rem'}/>
                            </button>
                        </Show>
                    </Show>
                </div>
            </Portal>
            
            <div class="bg-white rounded-lg shadow p-6 mb-6">
                <Show when={!editMode()} fallback={
                    <>
                        <div class="mb-4">
                            <h3 class="text-lg font-semibold mb-2">Package Name</h3>
                            <input 
                                type="text"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                value={editName()}
                                onInput={(e) => setEditName(e.target.value)}
                                placeholder="Package name"
                            />
                        </div>
                        
                        <div class="mb-4">
                            <h3 class="text-lg font-semibold mb-2">Icon URL</h3>
                            <input 
                                type="text"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                value={editIcon()}
                                onInput={(e) => setEditIcon(e.target.value)}
                                placeholder="Icon URL (optional)"
                            />
                            <Show when={editIcon()}>
                                <div class="mt-2">
                                    <img src={editIcon()} alt="Icon preview" class="w-16 h-16 rounded" />
                                </div>
                            </Show>
                        </div>
                        
                        <div class="mb-4">
                            <h3 class="text-lg font-semibold mb-2">Description</h3>
                            <textarea 
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                rows="4"
                                value={editDescription()}
                                onInput={(e) => setEditDescription(e.target.value)}
                                placeholder="Package description"
                            />
                        </div>
                        
                        <Show when={saving()}>
                            <div class="text-blue-500 mb-4">Saving changes...</div>
                        </Show>
                    </>
                }>
                    <>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                            <div class="flex items-center gap-4">
                                <Show when={pkg.latest?.icon}>
                                    <img src={pkg.latest?.icon} alt={pkg.latest?.name} class="w-16 h-16 rounded" />
                                </Show>
                                <div>
                                    <h2 class="text-xl font-semibold">{pkg.latest?.name}</h2>
                                    {/* <p class="text-gray-600">ID: {pkg.latest?.id}</p> */}
                                </div>
                            </div>
                        </div>
                        
                        <div class="mb-4">
                            <h3 class="text-lg font-semibold mb-2">Description</h3>
                            <p class="text-gray-700">{pkg.latest?.description || 'No description provided'}</p>
                        </div>
                    </>
                </Show>
            </div>
            
            {/* Package Items Sections */}
            <div class="space-y-6">
                {/* Node Shapes Section */}
                <div class="bg-white rounded-lg shadow p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-xl font-bold">Node Shapes</h2>
                        <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1">
                            <Plus size={'0.8rem'}/>
                            Add Shape
                        </button>
                    </div>
                    <div class="text-gray-500">No node shapes defined yet.</div>
                </div>
                
                {/* Connection Lines Section */}
                <div class="bg-white rounded-lg shadow p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-xl font-bold">Connection Lines</h2>
                        <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1">
                            <Plus size={'0.8rem'}/>
                            Add Line
                        </button>
                    </div>
                    <div class="text-gray-500">No connection lines defined yet.</div>
                </div>
                
                {/* Connection Rules Section */}
                <div class="bg-white rounded-lg shadow p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-xl font-bold">Connection Rules</h2>
                        <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1">
                            <Plus size={'0.8rem'}/>
                            Add Rule
                        </button>
                    </div>
                    <div class="text-gray-500">No connection rules defined yet.</div>
                </div>
            </div>
                                
            
            {/* Delete Confirmation Modal */}
            <Show when={showDeleteConfirm()}>
                <Portal mount={document.querySelector('main')!}>
                    <ModalCentered>
                        <div class="p-4">
                            <h3 class="text-lg font-bold mb-4">Confirm Delete</h3>
                            <p class="mb-4">Are you sure you want to delete the package "{pkg.latest?.name}"? This action cannot be undone.</p>
                            <div class="flex gap-2 justify-end">
                                <button 
                                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
                                    onClick={cancelDelete}
                                >
                                    Cancel
                                </button>
                                <button 
                                    class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
                                    onClick={handleDelete}
                                >
                                    Delete
                                </button>
                            </div>
                        </div>
                    </ModalCentered>
                </Portal>
            </Show>
        </>
    );
}

export default function Package() {
    const params = useParams();
    const packageId = params.id;
    
    if (!packageId) {
        return (
            <div class="p-4">
                <div class="error">
                    <h2>Error</h2>
                    <p>Package ID is required</p>
                </div>
            </div>
        );
    }

    return (
        <ErrorBoundary fallback={(error) => (
            <div class="p-4">
                <div class="error">
                    <h2>Error loading package</h2>
                    <p>{error.message}</p>
                </div>
            </div>
        )}>
            <PackageItemProvider packageId={packageId}>
                <Suspense fallback={<div class="p-4">Loading package...</div>}>
                    <PackageContent />
                </Suspense>
            </PackageItemProvider>
        </ErrorBoundary>
    );
}
