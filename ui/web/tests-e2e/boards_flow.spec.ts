import { test, expect, type Page, type Locator } from '@playwright/test';
import { apiSignInAndApplyCookie } from './helpers/auth';

// Mirrors package_flow.spec.ts for boards

type BoardLocators = {
  addButton: Locator;
  modal: Locator;
  form: Locator;
  nameInput: Locator;
  descriptionInput: Locator;
  iconInput: Locator;
  isPublicCheckbox: Locator;
  list: Locator;
};

async function openBoards(page: Page) {
  await page.goto('/board');
}

function getNewBoardLocators(page: Page): BoardLocators {
  const form = page.getByTestId('board-new-form');
  return {
    addButton: page.getByTestId('board-add-button'),
    modal: page.getByTestId('modal-centered'),
    form,
    nameInput: form.locator('input[name="name"]'),
    descriptionInput: form.locator('textarea[name="description"]'),
    iconInput: form.locator('input[name="icon"]'),
    isPublicCheckbox: form.locator('input[name="is_public"]'),
    list: page.getByRole('list'),
  };
}

async function openNewBoardModal(locators: BoardLocators) {
  const { addButton, modal, form } = locators;
  await expect(addButton).toBeVisible();
  await addButton.click();
  await expect(modal).toBeVisible();
  await expect(form).toBeVisible();
}

async function fillBoardForm(locators: BoardLocators, { name, description, icon, isPublic }: { name: string; description?: string; icon?: string; isPublic?: boolean; }) {
  await locators.nameInput.fill(name);
  if (description !== undefined) await locators.descriptionInput.fill(description);
  if (icon !== undefined) await locators.iconInput.fill(icon);
  if (typeof isPublic === 'boolean') {
    const checked = await locators.isPublicCheckbox.isChecked();
    if (!!isPublic !== checked) await locators.isPublicCheckbox.click();
  }
}

async function submitBoardForm(locators: BoardLocators) {
  await locators.form.locator('button[type="submit"]').click();
  await expect(locators.modal).toBeHidden();
}

function queryItemByName(locators: BoardLocators, name: string): Locator {
  return locators.list.getByRole('listitem').filter({ hasText: name });
}

async function openItemDetailFromRow(row: Locator) {
  await row.getByRole('link').click();
}

function getEditFormLocators(page: Page) {
  const form = page.getByTestId('board-edit-form');
  return {
    form,
    nameInput: form.locator('input[name="name"]'),
    descriptionInput: form.locator('textarea[name="description"]'),
    iconInput: form.locator('input[name="icon"]'),
    isPublicCheckbox: form.locator('input[name="is_public"]'),
    saveButton: page.getByTestId('board-save-button'),
    editButton: page.getByTestId('board-edit-button'),
    deleteButton: page.getByTestId('board-delete-button'),
    cancelButton: page.getByTestId('board-cancel-button'),
  };
}

// Tests

test.describe('Board flow', () => {
  test.beforeEach(async ({ request, context }) => {
    await apiSignInAndApplyCookie(request, context, 'e2e@example.com', 'some_code');
  });

  test('create private board', async ({ page }) => {
    const locators = getNewBoardLocators(page);
    await openBoards(page);
    await openNewBoardModal(locators);

    const name = `brd-private-${Date.now()}`;
    await fillBoardForm(locators, { name, description: 'private board via e2e', isPublic: false });

    await submitBoardForm(locators);
    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();
    await expect(row).toContainText('Private');
  });

  test('create public board', async ({ page }) => {
    const locators = getNewBoardLocators(page);
    await openBoards(page);
    await openNewBoardModal(locators);

    const name = `brd-public-${Date.now()}`;
    await fillBoardForm(locators, { name, description: 'public board via e2e', isPublic: true });

    await submitBoardForm(locators);
    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();
    await expect(row).toContainText('Public');
  });

  test('update private board (make it public and change fields)', async ({ page }) => {
    const locators = getNewBoardLocators(page);
    await openBoards(page);
    await openNewBoardModal(locators);

    const name = `brd-upd-private-${Date.now()}`;
    await fillBoardForm(locators, { name, description: 'before update (private)', isPublic: false });
    await submitBoardForm(locators);

    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();
    await openItemDetailFromRow(row);

    const edit = getEditFormLocators(page);
    await edit.editButton.click();

    await edit.nameInput.fill(`${name}-edited`);
    await edit.iconInput.fill('');
    await edit.descriptionInput.fill('after update (public)');
    if (!(await edit.isPublicCheckbox.isChecked())) await edit.isPublicCheckbox.click();

    await edit.saveButton.click();

    await expect(page.getByTestId('board-detail-name')).toContainText(`${name}-edited`);
    await expect(page.getByTestId('board-detail-visibility')).toHaveText('Public');

    await page.goto('/board');
    const updatedRow = queryItemByName(locators, `${name}-edited`);
    await expect(updatedRow).toBeVisible();
    await expect(updatedRow).toContainText('Public');
  });

  test('update public board (make it private and change fields)', async ({ page }) => {
    const locators = getNewBoardLocators(page);
    await openBoards(page);
    await openNewBoardModal(locators);

    const name = `brd-upd-public-${Date.now()}`;
    await fillBoardForm(locators, { name, description: 'before update (public)', isPublic: true });
    await submitBoardForm(locators);

    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();
    await openItemDetailFromRow(row);

    const edit = getEditFormLocators(page);
    await edit.editButton.click();

    await edit.nameInput.fill(`${name}-edited`);
    await edit.iconInput.fill('');
    await edit.descriptionInput.fill('after update (private)');
    if (await edit.isPublicCheckbox.isChecked()) await edit.isPublicCheckbox.click();

    await edit.saveButton.click();

    await expect(page.getByTestId('board-detail-name')).toContainText(`${name}-edited`);
    await expect(page.getByTestId('board-detail-visibility')).toHaveText('Private');

    await page.goto('/board');
    const updatedRow = queryItemByName(locators, `${name}-edited`);
    await expect(updatedRow).toBeVisible();
    await expect(updatedRow).toContainText('Private');
  });

  test('delete private board', async ({ page }) => {
    const locators = getNewBoardLocators(page);
    await openBoards(page);
    await openNewBoardModal(locators);

    const name = `brd-del-private-${Date.now()}`;
    await fillBoardForm(locators, { name, description: 'to be deleted (private)', isPublic: false });
    await submitBoardForm(locators);

    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();
    await openItemDetailFromRow(row);

    const edit = getEditFormLocators(page);
    await edit.deleteButton.click();

    const modal = page.getByTestId('modal-centered');
    await expect(modal).toBeVisible();
    await page.getByTestId('modal-delete-button').click();

    await expect(page).toHaveURL(/\/board$/);
    await expect(queryItemByName(locators, name)).toHaveCount(0);
  });

  test('delete public board', async ({ page }) => {
    const locators = getNewBoardLocators(page);
    await openBoards(page);
    await openNewBoardModal(locators);

    const name = `brd-del-public-${Date.now()}`;
    await fillBoardForm(locators, { name, description: 'to be deleted (public)', isPublic: true });
    await submitBoardForm(locators);

    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();
    await openItemDetailFromRow(row);

    const edit = getEditFormLocators(page);
    await edit.deleteButton.click();

    const modal = page.getByTestId('modal-centered');
    await expect(modal).toBeVisible();
    await page.getByTestId('modal-delete-button').click();

    await expect(page).toHaveURL(/\/board$/);
    await expect(queryItemByName(locators, name)).toHaveCount(0);
  });
});
