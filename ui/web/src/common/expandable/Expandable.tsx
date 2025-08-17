import { createSignal, type ParentComponent, Show } from 'solid-js';

type Props = {
  title: string;
  openByDefault?: boolean;
  testIdPrefix?: string;
};

export const Expandable: ParentComponent<Props> = (props) => {
  const [open, setOpen] = createSignal(props.openByDefault ?? true);
  const p = (s: string) => (props.testIdPrefix ? `${props.testIdPrefix}-${s}` : undefined);

  return (
    <div class="border rounded-lg">
      <button
        type="button"
        class="w-full flex items-center justify-between px-4 py-2 bg-gray-50 hover:bg-gray-100 rounded-t-lg"
        onClick={() => setOpen(!open())}
        data-testid={p('toggle')}
        aria-expanded={open()}
      >
        <span class="text-lg font-bold">{props.title}</span>
        <span class="text-sm text-gray-600">{open() ? '−' : '+'}</span>
      </button>
      <Show when={open()}>
        <div class="p-6" data-testid={p('content')}>
          {props.children}
        </div>
      </Show>
    </div>
  );
};

export default Expandable;
