import { test, expect, type Page, type Locator } from '@playwright/test';
import { apiSignInAndApplyCookie } from './helpers/auth';

type PackageLocators = {
  addButton: Locator;
  modal: Locator;
  form: Locator;
  nameInput: Locator;
  descriptionInput: Locator;
  iconInput: Locator;
  isPublicCheckbox: Locator;
  list: Locator;
};

// Helpers
async function openPackages(page: Page) {
  await page.goto('/package');
}

function getNewPackageLocators(page: Page): PackageLocators {
  const form = page.getByTestId('package-new-form');
  return {
    addButton: page.getByTestId('package-add-button'),
    modal: page.getByTestId('modal-centered'), // Adjusted to match the modal test ID
    form,
    nameInput: form.locator('input[name="name"]'),
    descriptionInput: form.locator('textarea[name="description"]'),
    iconInput: form.locator('input[name="icon"]'),
    isPublicCheckbox: form.locator('input[name="is_public"]'),
    list: page.getByTestId('package-items'),
  };
}

async function openNewPackageModal(locators: PackageLocators) {
  const { addButton, modal, form } = locators;
  await expect(addButton).toBeVisible();
  await addButton.click();
  await expect(modal).toBeVisible();
  await expect(form).toBeVisible();
}

async function fillPackageForm(locators: PackageLocators, { name, description, icon, isPublic }: { name: string; description?: string; icon?: string; isPublic?: boolean; }) {
  await locators.nameInput.fill(name);
  if (description !== undefined) {
    await locators.descriptionInput.fill(description);
  }
  if (icon !== undefined) {
    await locators.iconInput.fill(icon);
  }
  if (typeof isPublic === 'boolean') {
    const checked = await locators.isPublicCheckbox.isChecked();
    if (!!isPublic !== checked) {
      await locators.isPublicCheckbox.click();
    }
  }
}

async function submitPackageForm(locators: PackageLocators) {
  await locators.form.locator('button[type="submit"]').click();
  await expect(locators.modal).toBeHidden();
}

function queryItemByName(locators: PackageLocators, name: string): Locator {
  return locators.list.getByRole('listitem').filter({ hasText: name });
}

// Tests

test.describe('Package flow', () => {
  test.beforeEach(async ({ request, context }) => {
    await apiSignInAndApplyCookie(request, context, 'e2e@example.com', 'some_code');
  });

  test('create private package', async ({ page }) => {
    const locators = getNewPackageLocators(page);
    await openPackages(page);
    await openNewPackageModal(locators);

    const name = `pkg-private-${Date.now()}`;
    await fillPackageForm(locators, {
      name,
      description: 'private package via e2e',
      icon: '',
      isPublic: false,
    });

    await submitPackageForm(locators);
    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();

    const visibility = row.getByTestId('package-item-visibility');
    await expect(visibility).toBeVisible();
    await expect(visibility).toHaveText('Private');
  });

  test('create public package', async ({ page }) => {
    const locators = getNewPackageLocators(page);
    await openPackages(page);
    await openNewPackageModal(locators);

    const name = `pkg-public-${Date.now()}`;
    await fillPackageForm(locators, {
      name,
      description: 'public package via e2e',
      icon: '',
      isPublic: true,
    });

    await submitPackageForm(locators);
    const row = queryItemByName(locators, name);
    await expect(row).toBeVisible();
    const visibility = row.getByTestId('package-item-visibility');
    await expect(visibility).toBeVisible();
    await expect(visibility).toHaveText('Public');
  });
});
