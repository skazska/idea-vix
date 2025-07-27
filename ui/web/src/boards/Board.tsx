import { useParams, A, useNavigate } from "@solidjs/router";
import { createSignal, Show, ErrorBoundary, Suspense, createEffect } from "solid-js";
import { setTitle } from "../common/providers/page-state";
import { BoardItemProvider, useBoardItem } from "./providers/item";
import { Portal } from "solid-js/web";
import { Edit, Trash2, ArrowLeft, Save, X } from "lucide-solid";
import { ModalCentered } from "../common/modals";
import type { TBoard } from "./model";
import { ENTITY_NAME, PAGE_TITLE, ROUTE } from "./const";

function BoardContent() {
    console.log("BoardContent rendered");
    const navigate = useNavigate();
    const params = useParams();
    const boardId = params.id;
    
    const [brd, actions] = useBoardItem();
    const [editMode, setEditMode] = createSignal(false);
    const [showDeleteConfirm, setShowDeleteConfirm] = createSignal(false);
    const [saving, setSaving] = createSignal(false);
    
    // Edit form fields
    const [editName, setEditName] = createSignal("");
    const [editDescription, setEditDescription] = createSignal("");
    const [editIcon, setEditIcon] = createSignal("");
    
    const handleEdit = () => {
        // Initialize edit form with current values
        const currentBrd = brd.latest;
        if (currentBrd) {
            setEditName(currentBrd.name);
            setEditDescription(currentBrd.description || "");
            setEditIcon(currentBrd.icon || "");
        }
        setEditMode(true);
    };
    
    const handleSave = async () => {
        if (!boardId) return;
        
        setSaving(true);
        try {
            const updates: Partial<Omit<TBoard, 'id'>> = {
                name: editName().trim(),
                description: editDescription().trim(),
                icon: editIcon().trim() || undefined,
            };
            
            await actions.update(boardId, updates);
            setEditMode(false);
            // Reload to get fresh data
            actions.reload();
        } catch (error) {
            console.error(`Failed to save ${ENTITY_NAME}:`, error);
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
        if (!boardId) return;
        try {
            await actions.remove(boardId);
            navigate(ROUTE);
        } catch (error) {
            console.error(`Failed to delete ${ENTITY_NAME}:`, error);
        }
    };
    
    const confirmDelete = () => {
        setShowDeleteConfirm(true);
    };
    
    const cancelDelete = () => {
        setShowDeleteConfirm(false);
    };

    createEffect(() => {
        setTitle(`${PAGE_TITLE} ${brd.latest?.name}`);
    });

    return (
        <>
            {/* Navigation and action buttons in header */}
            <Portal mount={document.getElementById("sub-menu")!}>
                <div class="flex gap-2">
                    <A href={ROUTE} class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-1 px-2 rounded text-m">
                        <ArrowLeft size={'1rem'}/>
                    </A>
                    <Show when={brd.latest}>
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
                            <h3 class="text-lg font-semibold mb-2">Name</h3>
                            <input 
                                type="text"
                                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                value={editName()}
                                onInput={(e) => setEditName(e.target.value)}
                                placeholder={`name`}
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
                                placeholder={`description`}
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
                                <Show when={brd.latest?.icon}>
                                    <img src={brd.latest?.icon} alt={brd.latest?.name} class="w-16 h-16 rounded" />
                                </Show>
                                <div>
                                    <h2 class="text-xl font-semibold">{brd.latest?.name}</h2>
                                </div>
                            </div>
                        </div>
                        
                        <div class="mb-4">
                            <h3 class="text-lg font-semibold mb-2">Description</h3>
                            <p class="text-gray-700">{brd.latest?.description || 'No description provided'}</p>
                        </div>
                    </>
                </Show>
            </div>
            
            {/* Delete Confirmation Modal */}
            <Show when={showDeleteConfirm()}>
                <Portal mount={document.querySelector('main')!}>
                    <ModalCentered>
                        <div class="p-4">
                            <h3 class="text-lg font-bold mb-4">Confirm Delete</h3>
                            <p class="mb-4">Are you sure you want to delete the {ENTITY_NAME} "{brd.latest?.name}"? This action cannot be undone.</p>
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

export default function Board() {
    const params = useParams();
    const boardId = params.id;
    
    if (!boardId) {
        return (
            <div class="p-4">
                <div class="error">
                    <h2>Error</h2>
                    <p>Board ID is required</p>
                </div>
            </div>
        );
    }

    return (
        <ErrorBoundary fallback={(error) => (
            <div class="p-4">
                <div class="error">
                    <h2>Error loading board</h2>
                    <p>{error.message}</p>
                </div>
            </div>
        )}>
            <BoardItemProvider boardId={boardId}>
                <Suspense fallback={<div class="p-4">Loading board...</div>}>
                    <BoardContent />
                </Suspense>
            </BoardItemProvider>
        </ErrorBoundary>
    );
}
