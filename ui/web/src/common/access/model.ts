export type AccessRole = 'owner' | 'manage' | 'edit' | 'view';

export type GrantRequest = {
  address: string;
  role: Exclude<AccessRole, 'owner'>; // cannot grant owner via UI
};

export type AccessMapping = {
  // resource id field name differs per entity; we don't strictly depend on it in UI
  address: string;
  role: AccessRole | string; // backend returns string; normalize for display
};

export const grantableRoles: GrantRequest['role'][] = ['view', 'edit', 'manage'];
