// Component - wrapper of content visible when request function does not return Forbidden or Unauthorized
// prop: children - content to render when access is granted
// prop: fallback - content to render when access is denied
// prop: resource - access model

import { Show, type JSXElement, type ParentComponent } from "solid-js";
import { useAccess } from "./access.provider";
import type { AccessRole } from "./model";

const Accessible: ParentComponent<{ roles: AccessRole[]; fallback?: JSXElement }> = (props) => {
    const { children, fallback } = props;
    const [access, { reload: _ }] = useAccess();

    const hasAccess = () => access && props.roles.some(role => access()?.includes(role));

    return (
        <Show when={hasAccess()} fallback={fallback}>
            {children}
        </Show>
    )
}

export default Accessible;