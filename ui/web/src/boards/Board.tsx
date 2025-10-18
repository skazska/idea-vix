import { useParams, A, useNavigate, revalidate } from "@solidjs/router";
import { createSignal, Show, ErrorBoundary, Suspense, createEffect, Switch, Match } from "solid-js";
import { setTitle } from "../common/providers/page-state";
import { BoardItemProvider, useBoardItem } from "./providers/item.provider";
import { Portal } from "solid-js/web";
import { Edit, Trash2, ArrowLeft, Save, X, Users } from "lucide-solid";
import { ModalCentered } from "../common/modals";
import type { TBoard } from "./model";
import { ENTITY_NAME, PAGE_TITLE, ROUTE } from "./const";
import { AccessManager } from "../common/access/AccessManager";
import { getBoardApi } from "./providers/api";
import { useBackend } from "../common/providers/backend";
import Expandable from "../common/expandable/Expandable";
import { AccessProvider } from "../common/access/access.provider";
import { AccessMapProvider } from "../common/access/accessMap.provider";
import Accessible from "../common/access/Accessible";
import { WorkshopManagerProvider, WorkshopManager } from "../common/workshop";
import BoardPackages from "./BoardPackages";

function BoardContent() {
    console.log("BoardContent rendered");
    const navigate = useNavigate();
    const params = useParams();
    const boardId = params.id;
    
    const [brd, actions] = useBoardItem();
    const boardApi = getBoardApi(useBackend());
    const [editMode, setEditMode] = createSignal(false);
    const [showDeleteConfirm, setShowDeleteConfirm] = createSignal(false);
    const [saving, setSaving] = createSignal(false);
    
    // Edit form fields
    const [editName, setEditName] = createSignal("");
    const [editDescription, setEditDescription] = createSignal("");
    const [editIcon, setEditIcon] = createSignal("");
    const [editIsPublic, setEditIsPublic] = createSignal(false);
    
    const handleEdit = () => {
        // Initialize edit form with current values
        const currentBrd = brd.latest;
        if (currentBrd) {
            setEditName(currentBrd.name);
            setEditDescription(currentBrd.description || "");
            setEditIcon(currentBrd.icon || "");
            setEditIsPublic(!!currentBrd.is_public);
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
                is_public: editIsPublic(),
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
        setEditIsPublic(false);
    };
    
    const handleDelete = async () => {
        if (!boardId) return;
        try {
            await actions.remove(boardId);
            revalidate(boardApi.list.key);
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
                                    data-testid="board-save-button"
                                    class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-m"
                                    onClick={handleSave}
                                    disabled={saving()}
                                >
                                    <Save size={'1rem'}/>
                                </button>
                                <button 
                                    data-testid="board-cancel-button"
                                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-1 px-2 rounded text-m"
                                    onClick={handleCancelEdit}
                                    disabled={saving()}
                                >
                                    <X size={'1rem'}/>
                                </button>
                            </>
                        }>
                            <button 
                                data-testid="board-edit-button"
                                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-m"
                                onClick={handleEdit}
                            >
                                <Edit size={'1rem'}/>
                            </button>
                            <button 
                                data-testid="board-delete-button"
                                class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-m"
                                onClick={confirmDelete}
                            >
                                <Trash2 size={'1rem'}/>
                            </button>
                        </Show>
                    </Show>
                </div>
            </Portal>

            <Show when={brd.latest} fallback={<div class="bg-white rounded-lg shadow p-6 mb-6">No board selected</div>}>
                {(b) => (
                    <AccessProvider id={b().id} api={boardApi}>
                        <Switch>
                            <Match when={editMode()}>
                                <div data-testid="board-edit-form" class="bg-white rounded-lg shadow p-6 mb-6">
                                    <div class="mb-4">
                                        <h3 class="text-lg font-semibold mb-2">Name</h3>
                                        <input 
                                            name="name"
                                            type="text"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                            value={editName()}
                                            onInput={(e) => setEditName(e.target.value)}
                                            placeholder={`name`}
                                        />
                                    </div>
                                    
                                    <div class="mb-4">
                                        <h3 class="text-lg font-semibold mb-2">Slug</h3>
                                        <input 
                                            type="text"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-100 text-gray-600"
                                            value={b().slug}
                                            disabled
                                            placeholder="Slug cannot be changed after creation"
                                        />
                                        <p class="text-xs text-gray-500 mt-1">Slug cannot be changed after creation</p>
                                    </div>
                                    
                                    <div class="mb-4">
                                        <h3 class="text-lg font-semibold mb-2">Icon URL</h3>
                                        <input 
                                            name="icon"
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
                                            name="description"
                                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                            rows="4"
                                            value={editDescription()}
                                            onInput={(e) => setEditDescription(e.target.value)}
                                            placeholder={`description`}
                                        />
                                    </div>

                                    <div class="mb-4">
                                        <label class="inline-flex items-center gap-2 select-none">
                                            <input
                                                name="is_public"
                                                type="checkbox"
                                                class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                                checked={editIsPublic()}
                                                onChange={(e) => setEditIsPublic(e.currentTarget.checked)}
                                            />
                                            <span class="text-gray-700">Public</span>
                                        </label>
                                    </div>
                                    
                                    <Show when={saving()}>
                                        <div class="text-blue-500 mb-4">Saving changes...</div>
                                    </Show>
                                </div>
                            </Match>
                            <Match when={!editMode()}>
                                <div class="bg-white rounded-lg shadow p-6 mb-6">
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                                        <div class="flex items-center gap-4">
                                            <Show when={b().icon}>
                                                <img src={b().icon} alt={b().name} class="w-16 h-16 rounded" />
                                            </Show>
                                            <div>
                                                <h2 class="text-xl font-semibold flex items-center gap-2">
                                                    <span data-testid="board-detail-name">{b().name}</span>
                                                    <span data-testid="board-detail-visibility" class={`text-xs px-2 py-1 rounded ${b().is_public ? 'bg-green-100 text-green-700' : 'bg-gray-200 text-gray-700'}`}>
                                                        {b().is_public ? 'Public' : 'Private'}
                                                    </span>
                                                </h2>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="mb-4">
                                        <h3 class="text-lg font-semibold mb-2">Slug</h3>
                                        <p class="text-gray-700 font-mono text-sm bg-gray-100 px-3 py-2 rounded">{b().slug}</p>
                                    </div>
                                    
                                    <div class="mb-4">
                                        <h3 class="text-lg font-semibold mb-2">Description</h3>
                                        <p class="text-gray-700">{b().description || 'No description provided'}</p>
                                    </div>
                                </div>

                                {/* Board Sections */}
                                <div class="space-y-6">
                                    {/* Access management */}
                                    <Accessible roles={["owner", "manage"]}>
                                        <Expandable title={(<h2 class="text-xl font-bold flex items-center gap-2"><Users size={'1rem'}/> Access</h2>)} openByDefault={false} name="board-section-access">
                                            <AccessMapProvider id={b().id} api={boardApi}>
                                                <AccessManager id={b().id}/>
                                            </AccessMapProvider>
                                        </Expandable>
                                    </Accessible>

                                    {/* Packages Section */}
                                    <Accessible roles={["owner"]}>
                                        <Expandable title="Packages" openByDefault={false} name="board-section-packages">
                                            <BoardPackages boardId={boardId!} />
                                        </Expandable>
                                    </Accessible>

                                    {/* Workshop Items Section */}
                                    <Accessible roles={["owner", "manage", "edit"]}>
                                        <Expandable title="Workshop Items" openByDefault={false} name="board-section-workshop">
                                            <div class="bg-white rounded-lg shadow p-6">
                                                <WorkshopManagerProvider entityType="board" entityId={boardId!}>
                                                    <WorkshopManager entityType="board" entityId={boardId!} />
                                                </WorkshopManagerProvider>
                                            </div>
                                        </Expandable>
                                    </Accessible>
                                </div>
                            </Match>
                        </Switch>
                        {/* Delete Confirmation Modal */}
                        <Show when={showDeleteConfirm()}>
                            <Portal mount={document.querySelector('main')!}>
                                <ModalCentered>
                                    <div class="p-4">
                                        <h3 class="text-lg font-bold mb-4">Confirm Delete</h3>
                                        <p class="mb-4">Are you sure you want to delete the {ENTITY_NAME} "{b().name}"? This action cannot be undone.</p>
                                        <div class="flex gap-2 justify-end">
                                            <button 
                                                data-testid="modal-cancel-button"
                                                class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
                                                onClick={cancelDelete}
                                            >
                                                Cancel
                                            </button>
                                            <button 
                                                data-testid="modal-delete-button"
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
                    </AccessProvider>
                )}
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
