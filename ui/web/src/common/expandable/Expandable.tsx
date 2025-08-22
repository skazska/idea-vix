import { FoldVertical, UnfoldVertical } from 'lucide-solid';
import { createSignal, type JSXElement, Match, type ParentComponent, Show, Switch } from 'solid-js';

type Props = {
  title: string | JSXElement;
  description?: string | JSXElement;
  openByDefault?: boolean;
  name?: string;
};

export const Expandable: ParentComponent<Props> = (props) => {
  const [open, setOpen] = createSignal(props.openByDefault ?? false);
  const p = (s: string) => (props.name ? `${props.name}-${s}` : undefined);

  return (
    <div class="border rounded-lg" role="region" data-testid={props.name}>
      <button
        type="button"
        class="w-full flex items-center justify-between px-4 py-2 bg-gray-50 hover:bg-gray-100 rounded-t-lg"
        onClick={() => setOpen(!open())}
        data-testid={p('toggle')}
        aria-expanded={open()}
      >
        <span class="flex-shrink-1">
          <Switch fallback={<h3 class="text-lg font-bold">{props.title}</h3>}>
            <Match when={typeof props.title === 'function'}>
              {props.title}
            </Match>
          </Switch>
        </span>

        <span class="flex-grow-1 text-sm text-gray-600">{props.description}</span>
        <span class="flex-shrink-1 text-sm text-gray-600">{open() ? (<FoldVertical />) : (<UnfoldVertical />)}</span>
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
