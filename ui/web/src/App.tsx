/**
 * App component
 * @file ui/web/src/App.tsx
 * @author ska
 *
 * This file defines root application component for the SolidJS app.
 * 
 * It sets up the navigation, header, footer, and main content area.
 *
 * Header contains the title and sub-menu, while the footer contains status and copyright information.
 * Menu consists of 1st level routes to pages.
 */

import { For, Suspense, type ParentComponent } from "solid-js";
import { usePageState } from "./common/providers/page-state";
import SessionIcon from "./common/session/SessionIcon";
import { APP_COPYRIGHT } from "./common/const";
import { ROUTE as BOARD_ROUTE, PAGE_TITLE as BOARD_TITLE } from "./boards/const";
import { ROUTE as PACKAGE_ROUTE, PAGE_TITLE as PACKAGE_TITLE } from "./package/const";

const pages = {
    "/": "Home",
    [PACKAGE_ROUTE]: PACKAGE_TITLE,
    [BOARD_ROUTE]: BOARD_TITLE,
    // "/learn": "Learn",
}

const App: ParentComponent = (props) => {
    const [pageState] = usePageState(); 

    return (<>
        <nav class="flex flex-shrink-0 items-center justify-between p-2 bg-gray-800 text-white">
            <header class="flex items-center w-full">
                <h1 id="title" class="pr-4">{pageState.title()}</h1>
                <div id='sub-menu' class="flex items-left space-x-4"></div>
            </header>
            <ul id="main-menu" class="flex space-x-4 items-center">
                <For each={Object.entries(pages)}>
                    {([path, name]) => (
                        <li><a href={path} class="hover:underline">{name}</a></li>
                    )}
                </For>
                <li>
                    <SessionIcon />
                </li>
            </ul>
        </nav>
        <main class="flex-grow overflow-auto scrollbar-thin [scrollbar-gutter:stable]">
            <Suspense fallback={<div class="p-4">Loading...</div>}>
                <div class="p-4">
                    {props.children}
                </div>
            </Suspense>
        </main>
        <footer class="flex flex-shrink-0 justify-between flex-row items-center p-1 bg-gray-800 text-white">
            <p class="text-xs">status</p>
            <p>{APP_COPYRIGHT}</p>
        </footer>
    </>);
}

export default App;
