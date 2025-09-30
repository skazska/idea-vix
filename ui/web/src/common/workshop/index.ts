// Workshop module - Complete workshop item management system
// Provides models, API functions, providers, and components for managing
// workshop items (shapes, lines, rules, layouts) within packages and boards

// Types and models
export type { 
    WorkshopItem, 
    NewWorkshopItem, 
    PatchWorkshopItem, 
    WorkshopItemType,
    ShapeDefinition,
    LineDefinition,
    RuleDefinition,
    LayoutDefinition
} from "./model";

export { 
    getWorkshopItemTypeDisplayName, 
    getWorkshopItemTypePluralDisplayName,
    validateWorkshopItemDefinition 
} from "./model";

// API functions
export type { WorkshopEntityType, IWorkshopApi } from "./api";
export { 
    createWorkshopApi, 
    createEntityWorkshopApi, 
    createGlobalWorkshopApi 
} from "./api";

// Context providers
export type { 
    WorkshopItemWithType,
    IWorkshopManagerContext, 
    IWorkshopItemsContext,
    WorkshopManagerProviderProps,
    WorkshopItemsProviderProps 
} from "./providers";

export { 
    WorkshopManagerProvider,
    WorkshopItemsProvider,
    useWorkshopManager, 
    useWorkshopItems 
} from "./providers";

// UI Components
export type { 
    WorkshopItemListProps, 
    WorkshopItemFormProps,
    WorkshopManagerProps 
} from "./components";

export { 
    WorkshopItemList, 
    WorkshopItemForm,
    WorkshopManager 
} from "./components";