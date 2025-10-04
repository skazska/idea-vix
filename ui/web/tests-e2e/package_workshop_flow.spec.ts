import { test, expect, type Page, type Locator } from '@playwright/test';
import { apiSignInAndApplyCookie } from './helpers/auth';

type WorkshopLocators = {
  section: Locator;
  toggle: Locator;
  manager: Locator;
  shapesTab: Locator;
  linesTab: Locator;
  rulesTab: Locator;
  layoutsTab: Locator;
  addButton: Locator;
  itemsList: Locator;
};

type WorkshopFormLocators = {
  modal: Locator;
  nameInput: Locator;
  slugInput: Locator;
  descriptionInput: Locator;
  definitionInput: Locator;
  saveButton: Locator;
  cancelButton: Locator;
};

// Helper functions
async function createTestPackage(page: Page): Promise<string> {
  await page.goto('/package');
  
  // Create a test package using existing patterns
  const packageName = `workshop-test-pkg-${Date.now()}`;
  
  // Open new package modal
  await page.getByTestId('package-add-button').click();
  await expect(page.getByTestId('modal-centered')).toBeVisible();
  
  const form = page.getByTestId('package-new-form');
  await form.locator('input[name="name"]').fill(packageName);
  await form.locator('textarea[name="description"]').fill('Test package for workshop E2E tests');
  await form.locator('button[type="submit"]').click();
  
  // Wait for modal to close
  await expect(page.getByTestId('modal-centered')).toBeHidden();
  
  // Find the created package in the list and open it
  const packageList = page.getByTestId('package-items');
  const packageRow = packageList.getByRole('listitem').filter({ hasText: packageName });
  await expect(packageRow).toBeVisible();
  
  // Click to open package detail
  await packageRow.getByTestId('open-package-item').click();
  
  // Wait for navigation to package detail page
  await expect(page).toHaveURL(/\/package\/\d+/);
  const url = page.url();
  const packageId = url.split('/package/')[1];
  
  return packageId;
}

function getWorkshopLocators(page: Page): WorkshopLocators {
  const section = page.getByTestId('package-section-workshop');
  const manager = page.getByTestId('workshop-manager');
  
  return {
    section,
    toggle: page.getByTestId('package-section-workshop-toggle'),
    manager,
    shapesTab: page.getByTestId('workshop-tab-shapes'),
    linesTab: page.getByTestId('workshop-tab-lines'),
    rulesTab: page.getByTestId('workshop-tab-rules'),
    layoutsTab: page.getByTestId('workshop-tab-layouts'),
    addButton: manager.locator('[data-testid*="workshop-add-"]').first(),
    itemsList: manager.locator('[data-testid*="workshop-item-"]'),
  };
}

function getWorkshopFormLocators(page: Page): WorkshopFormLocators {
  const modal = page.locator('.fixed.inset-0'); // Modal backdrop
  
  return {
    modal,
    nameInput: page.getByTestId('workshop-form-name-input'),
    slugInput: page.getByTestId('workshop-form-slug-input'),
    descriptionInput: page.getByTestId('workshop-form-description-input'),
    definitionInput: page.getByTestId('workshop-form-definition-input'),
    saveButton: page.getByTestId('workshop-form-save-button'),
    cancelButton: page.getByTestId('workshop-form-cancel-button'),
  };
}

async function openWorkshopSection(locators: WorkshopLocators) {
  await expect(locators.section).toBeVisible();
  await locators.toggle.click();
  await expect(locators.manager).toBeVisible();
}

async function switchToItemType(locators: WorkshopLocators, itemType: 'shapes' | 'lines' | 'rules' | 'layouts') {
  const tab = locators[`${itemType}Tab` as keyof WorkshopLocators] as Locator;
  await tab.click();
  
  // Wait for the add button to update for the new item type
  await expect(locators.manager.getByTestId(`workshop-add-${itemType}-button`)).toBeVisible();
}

async function fillWorkshopItemForm(formLocators: WorkshopFormLocators, data: {
  name: string;
  description?: string;
  definition: object;
}) {
  // Wait for form to be ready
  await expect(formLocators.nameInput).toBeVisible();
  await expect(formLocators.definitionInput).toBeVisible();
  
  await formLocators.nameInput.fill(data.name);
  
  if (data.description) {
    await formLocators.descriptionInput.fill(data.description);
  }
  
  // Clear and fill definition JSON with some delay for reliability
  await formLocators.definitionInput.click();
  await formLocators.definitionInput.press('Control+a');
  await formLocators.definitionInput.fill(JSON.stringify(data.definition, null, 2));
  
  // Wait a bit for validation to complete
  await formLocators.nameInput.page().waitForTimeout(500);
}

async function createWorkshopItem(
  page: Page,
  workshopLocators: WorkshopLocators,
  itemType: 'shapes' | 'lines' | 'rules' | 'layouts',
  data: { name: string; description?: string; definition: object }
) {
  await switchToItemType(workshopLocators, itemType);
  
  // Click add button for the specific item type
  const addButton = page.getByTestId(`workshop-add-${itemType}-button`);
  await expect(addButton).toBeVisible();
  await addButton.click();
  
  // Fill form
  const formLocators = getWorkshopFormLocators(page);
  await expect(formLocators.modal).toBeVisible();
  
  await fillWorkshopItemForm(formLocators, data);
  
  // Click save and wait for response
  await formLocators.saveButton.click();
  
  // Give more time for the API request to complete
  await page.waitForTimeout(2000);
  
  // Wait for modal to close with longer timeout
  await expect(formLocators.modal).toBeHidden({ timeout: 10000 });
  
  // Verify item was created
  const itemSlug = data.name.toLowerCase().replace(/[^a-z0-9\s-]/g, '').replace(/\s+/g, '-');
  await expect(page.getByTestId(`workshop-item-${itemSlug}`)).toBeVisible({ timeout: 10000 });
  
  return itemSlug;
}

// Test data for different item types
const testDefinitions = {
  shapes: {
    background: {
      border: {
        path: "M0,0 L100,0 L100,50 L0,50 Z",
        stroke: { width: 2, color: "#ff0000" }
      }
    },
    label: {
      position: { x: 50, y: 25 },
      default_text: "Test Shape"
    }
  },
  lines: {
    stroke: {
      width: 3,
      color: "#00ff00",
      style: "dashed"
    },
    source_socket: "output",
    target_socket: "input"
  },
  rules: {
    conditions: {
      node_type: "process"
    },
    actions: {
      highlight: true
    }
  },
  layouts: {
    type: "grid",
    properties: {
      columns: 3,
      rows: 3,
      spacing: 20
    }
  }
};

// Tests
test.describe('Workshop Items Management', () => {
  test.beforeEach(async ({ request, context, page }) => {
    await apiSignInAndApplyCookie(request, context, 'e2e@example.com', 'some_code');
    await createTestPackage(page);
  });

  test('should open workshop section and display tabs', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    // Verify all tabs are visible
    await expect(workshopLocators.shapesTab).toBeVisible();
    await expect(workshopLocators.linesTab).toBeVisible();
    await expect(workshopLocators.rulesTab).toBeVisible();
    await expect(workshopLocators.layoutsTab).toBeVisible();
    
    // Verify default tab (shapes) is selected
    await expect(workshopLocators.shapesTab).toHaveClass(/border-blue-500/);
  });

  test('should create a shape item', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    const shapeName = `test-shape-${Date.now()}`;
    const itemSlug = await createWorkshopItem(page, workshopLocators, 'shapes', {
      name: shapeName,
      description: 'A test shape for E2E testing',
      definition: testDefinitions.shapes
    });
    
    // Verify item is displayed with correct information
    const item = page.getByTestId(`workshop-item-${itemSlug}`);
    await expect(item).toContainText(shapeName);
    await expect(item).toContainText('A test shape for E2E testing');
    
    // Verify action buttons are present
    await expect(page.getByTestId(`workshop-view-${itemSlug}`)).toBeVisible();
    await expect(page.getByTestId(`workshop-edit-${itemSlug}`)).toBeVisible();
    await expect(page.getByTestId(`workshop-delete-${itemSlug}`)).toBeVisible();
  });

  test('should create items of all types', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    // Create one item of each type with unique names
    const timestamp = Date.now();
    const itemSlugs = {
      shapes: await createWorkshopItem(page, workshopLocators, 'shapes', {
        name: `Test Shape ${timestamp}`,
        definition: testDefinitions.shapes
      }),
      lines: await createWorkshopItem(page, workshopLocators, 'lines', {
        name: `Test Line ${timestamp}`,
        definition: testDefinitions.lines
      }),
      rules: await createWorkshopItem(page, workshopLocators, 'rules', {
        name: `Test Rule ${timestamp}`,
        definition: testDefinitions.rules
      }),
      layouts: await createWorkshopItem(page, workshopLocators, 'layouts', {
        name: `Test Layout ${timestamp}`,
        definition: testDefinitions.layouts
      })
    };
    
    // Verify tab counts are updated
    for (const [itemType, slug] of Object.entries(itemSlugs)) {
      await switchToItemType(workshopLocators, itemType as any);
      await expect(page.getByTestId(`workshop-item-${slug}`)).toBeVisible();
      
      // Check that tab shows count of 1
      const tab = workshopLocators[`${itemType}Tab` as keyof WorkshopLocators] as Locator;
      await expect(tab).toContainText('1');
    }
  });

  test('should edit an existing workshop item', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    // Create a shape first
    const originalName = `Original Shape ${Date.now()}`;
    const itemSlug = await createWorkshopItem(page, workshopLocators, 'shapes', {
      name: originalName,
      description: 'Original description',
      definition: testDefinitions.shapes
    });
    
    // Click edit button
    await page.getByTestId(`workshop-edit-${itemSlug}`).click();
    
    // Verify form opens with existing data
    const formLocators = getWorkshopFormLocators(page);
    await expect(formLocators.modal).toBeVisible();
    await expect(formLocators.nameInput).toHaveValue(originalName);
    await expect(formLocators.descriptionInput).toHaveValue('Original description');
    
    // Modify the item
    await formLocators.nameInput.fill('Updated Shape');
    await formLocators.descriptionInput.fill('Updated description');
    
    // Update definition
    const updatedDefinition = {
      ...testDefinitions.shapes,
      background: {
        ...testDefinitions.shapes.background,
        border: {
          ...testDefinitions.shapes.background.border,
          stroke: { width: 5, color: "#0000ff" }
        }
      }
    };
    
    await formLocators.definitionInput.click();
    await formLocators.definitionInput.press('Control+a');
    await formLocators.definitionInput.fill(JSON.stringify(updatedDefinition, null, 2));
    
    await formLocators.saveButton.click();
    
    // Verify modal closes and item is updated
    await expect(formLocators.modal).toBeHidden();
    
    const item = page.getByTestId(`workshop-item-${itemSlug}`);
    await expect(item).toContainText('Updated Shape');
    await expect(item).toContainText('Updated description');
  });

  test('should delete a workshop item', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    // Create a shape first
    const itemSlug = await createWorkshopItem(page, workshopLocators, 'shapes', {
      name: `Shape to Delete ${Date.now()}`,
      definition: testDefinitions.shapes
    });
    
    // Setup dialog handler to auto-accept confirm dialogs
    page.on('dialog', dialog => dialog.accept());
    
    // Click delete button
    await page.getByTestId(`workshop-delete-${itemSlug}`).click();
    
    // Verify item is removed from the list
    await expect(page.getByTestId(`workshop-item-${itemSlug}`)).toHaveCount(0);
    
    // Verify tab count is updated (should show 0)
    await expect(workshopLocators.shapesTab).toContainText('0');
  });

  test('should validate form inputs', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    // Try to create a shape with invalid data
    await switchToItemType(workshopLocators, 'shapes');
    await page.getByTestId('workshop-add-shapes-button').click();
    
    const formLocators = getWorkshopFormLocators(page);
    await expect(formLocators.modal).toBeVisible();
    
    // Try to submit empty form
    await formLocators.saveButton.click();
    
    // Form should still be visible (validation failed)
    await expect(formLocators.modal).toBeVisible();
    
    // Fill name but leave definition invalid
    const testName = `Test Shape ${Date.now()}`;
    await formLocators.nameInput.fill(testName);
    await formLocators.definitionInput.fill('invalid json');
    await formLocators.saveButton.click();
    
    // Form should still be visible
    await expect(formLocators.modal).toBeVisible();
    
    // Fix the JSON and submit
    await formLocators.definitionInput.click();
    await formLocators.definitionInput.press('Control+a');
    await formLocators.definitionInput.fill(JSON.stringify(testDefinitions.shapes, null, 2));
    await formLocators.saveButton.click();
    
    // Now form should close and item should be created
    await expect(formLocators.modal).toBeHidden();
    const slugFromName = testName.toLowerCase().replace(/[^a-z0-9\s-]/g, '').replace(/\s+/g, '-');
    await expect(page.getByTestId(`workshop-item-${slugFromName}`)).toBeVisible();
  });

  test('should cancel form without saving', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    await switchToItemType(workshopLocators, 'shapes');
    await page.getByTestId('workshop-add-shapes-button').click();
    
    const formLocators = getWorkshopFormLocators(page);
    await expect(formLocators.modal).toBeVisible();
    
    // Fill some data
    await formLocators.nameInput.fill('Cancelled Shape');
    await formLocators.descriptionInput.fill('This should not be saved');
    
    // Cancel the form
    await formLocators.cancelButton.click();
    
    // Verify modal is closed and no item was created
    await expect(formLocators.modal).toBeHidden();
    await expect(page.getByTestId('workshop-item-cancelled-shape')).toHaveCount(0);
    
    // Verify tab still shows 0 items
    await expect(workshopLocators.shapesTab).toContainText('0');
  });

  test('should switch between item type tabs', async ({ page }) => {
    const workshopLocators = getWorkshopLocators(page);
    
    await openWorkshopSection(workshopLocators);
    
    // Create items of different types
    const timestamp = Date.now();
    await createWorkshopItem(page, workshopLocators, 'shapes', {
      name: `Test Shape ${timestamp}`,
      definition: testDefinitions.shapes
    });
    
    await createWorkshopItem(page, workshopLocators, 'lines', {
      name: `Test Line ${timestamp}`,
      definition: testDefinitions.lines
    });
    
    // Switch to shapes tab and verify only shape is shown
    await switchToItemType(workshopLocators, 'shapes');
    await expect(page.getByTestId(`workshop-item-test-shape-${timestamp}`)).toBeVisible();
    await expect(page.getByTestId(`workshop-item-test-line-${timestamp}`)).toHaveCount(0);
    
    // Switch to lines tab and verify only line is shown
    await switchToItemType(workshopLocators, 'lines');
    await expect(page.getByTestId(`workshop-item-test-line-${timestamp}`)).toBeVisible();
    await expect(page.getByTestId(`workshop-item-test-shape-${timestamp}`)).toHaveCount(0);
    
    // Verify add buttons change based on selected tab
    await expect(page.getByTestId('workshop-add-lines-button')).toBeVisible();
    await expect(page.getByTestId('workshop-add-shapes-button')).toHaveCount(0);
  });
});