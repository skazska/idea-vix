import { createSignal, For, Show, type Component } from 'solid-js';
import type { AccessMapping, GrantRequest } from './model';

export type AccessManagerProps = {
  // data
  load: () => Promise<AccessMapping[]>;
  grant: (item: GrantRequest) => Promise<AccessMapping>;
  revoke: (address: string) => Promise<AccessMapping>;
  // labels and ids for testing
  title?: string;
  testIdPrefix?: string; // e.g., "package-access"
};

export const AccessManager: Component<AccessManagerProps> = (props) => {
  const [items, setItems] = createSignal<AccessMapping[]>([]);
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<string | undefined>();
  const [forbidden, setForbidden] = createSignal(false);

  const [addr, setAddr] = createSignal('');
  const [role, setRole] = createSignal<GrantRequest['role']>('view');
  const [granting, setGranting] = createSignal(false);

  async function reload() {
    setLoading(true);
    setError(undefined);
    try {
      const list = await props.load();
      setItems(list);
    } catch (e: any) {
      const msg = e?.message || '';
      // Hide the whole access manager if user is not owner (server enforces owner-only on list)
      if (/403/.test(msg) || /Forbidden/i.test(msg)) {
        setForbidden(true);
        setError(undefined);
      } else {
        setError(msg || 'Failed to load');
      }
    } finally {
      setLoading(false);
    }
  }

  async function onGrant(ev: Event) {
    ev.preventDefault();
    setGranting(true);
    setError(undefined);
    try {
      await props.grant({ address: addr().trim(), role: role() });
      setAddr('');
      await reload();
    } catch (e: any) {
      setError(e?.message || 'Failed to grant');
    } finally {
      setGranting(false);
    }
  }

  async function onRevoke(address: string) {
    setError(undefined);
    try {
      await props.revoke(address);
      await reload();
    } catch (e: any) {
      setError(e?.message || 'Failed to revoke');
    }
  }

  // initial load
  reload();

  const p = (suffix: string) => (props.testIdPrefix ? `${props.testIdPrefix}-${suffix}` : undefined);

  return (
    <Show when={!forbidden()}>
    <div class="bg-white rounded-lg shadow p-4 space-y-4" data-testid={p('container')}>
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold">{props.title || 'Access'}</h3>
        <Show when={loading()}>
          <span class="text-blue-500 text-sm">Loading…</span>
        </Show>
      </div>
      <Show when={error()}>
        <div class="text-red-600" data-testid={p('error')}>{error()}</div>
      </Show>

      <form class="flex flex-col sm:flex-row gap-2 items-start sm:items-end" onSubmit={onGrant} data-testid={p('form')}>
        <div class="flex-1 w-full">
          <label class="block text-sm text-gray-700 mb-1">Address</label>
          <input name="address" class="border rounded w-full px-2 py-1" value={addr()} onInput={(e) => setAddr(e.currentTarget.value)} placeholder="user@example.com" />
        </div>
        <div>
          <label class="block text-sm text-gray-700 mb-1">Role</label>
          <select name="role" class="border rounded px-2 py-1" value={role()} onChange={(e) => setRole(e.currentTarget.value as GrantRequest['role'])}>
            <option value="view">view</option>
            <option value="edit">edit</option>
            <option value="manage">manage</option>
          </select>
        </div>
        <div>
          <button type="submit" class="bg-green-600 hover:bg-green-700 text-white px-3 py-1 rounded" disabled={granting()} data-testid={p('grant-button')}>
            Grant
          </button>
        </div>
      </form>

      <div>
        <div class="text-sm text-gray-600 mb-2">Current access</div>
        <div role="list" class="divide-y" data-testid={p('list')}>
          <For each={items()}>{(row) => (
            <div role="listitem" class="flex items-center justify-between py-2">
              <div class="space-x-2">
                <span class="font-mono" data-testid={p('address')}>{row.address}</span>
                <span class={`text-xxs px-1 py-0.5 rounded ${row.role === 'owner' ? 'bg-yellow-100 text-yellow-700' : row.role === 'manage' ? 'bg-blue-100 text-blue-700' : row.role === 'edit' ? 'bg-purple-100 text-purple-700' : 'bg-gray-200 text-gray-700'}`} data-testid={p('role')}>
                  {String(row.role)}
                </span>
              </div>
              <button class="bg-red-500 hover:bg-red-600 text-white px-2 py-1 rounded" onClick={() => onRevoke(row.address)} data-testid={p('revoke-button')}>
                Revoke
              </button>
            </div>
          )}</For>
          <Show when={items().length === 0 && !loading()}>
            <div class="text-gray-500">No access entries.</div>
          </Show>
        </div>
      </div>
    </div>
  </Show>
  );
};
