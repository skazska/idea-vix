/**
 * Routing for SolidJS Application
 * @file ui/web/src/Routing.tsx
 * @author ska
 * 
 * This file contains the routing logic for the SolidJS application.
 * 
 * It sets up the router, defines routes.
 */

import App from './App.tsx'
import { Route, Router, type RoutePreloadFuncArgs } from '@solidjs/router'
import { lazy, type Component } from 'solid-js'
import { getBoardApi } from './boards/providers/api.ts'
import type { TBoard } from './boards/model.ts'
import { useBackend } from './common/providers/backend.tsx'
import type { TPackage } from './package/model.ts'
import { getPackageApi } from './package/providers/api.ts'
import { ROUTE as BOARD_ROUTE } from './boards/const.ts'
import { ROUTE as PACKAGE_ROUTE } from './package/const.ts'

const Home = lazy(() => import('./home/App.tsx'))
const Package = lazy(() => import('./package/Package.tsx'))
const Packages = lazy(() => import('./package/Packages.tsx'))
const Board = lazy(() => import('./boards/Board.tsx'))
const Boards = lazy(() => import('./boards/Boards.tsx'))

function NotFound() {
    return (
        <div class="p-4">
            <h1>404 - Not Found</h1>
            <p>The page you are looking for does not exist.</p>
        </div>
    )
}

const Routing: Component = () => {
    const backend = useBackend();
    const packageApi = getPackageApi(backend);
    const boardApi = getBoardApi(backend);

    // function preloadPackages(): Promise<TPackage[]> {
    //     // return getPackageItems(useBackend())
    //     // return createAsync(() => packageApi.getPackages(), {  name: "packages-query" })
    //     return Promise.resolve([]);
    // }

    const preloadPackage = ({ params }: RoutePreloadFuncArgs): Promise<TPackage> => {
        return packageApi.getPackage(params.id)
    }

    const preloadBoards = (): Promise<TBoard[]> => {
        return boardApi.list()
    }

    const preloadBoard = ({ params }: RoutePreloadFuncArgs): Promise<TBoard> => {
        return boardApi.get(params.id)
    }

    return (
        <Router root={App}>
            <Route path="/" component={Home} />
            <Route path={PACKAGE_ROUTE}>
                <Route path="/" component={Packages}/>
                <Route path="/:id" component={Package} preload={preloadPackage}/>
            </Route>
            <Route path={BOARD_ROUTE}>
                <Route path="/" component={Boards} preload={preloadBoards} />
                <Route path="/:id" component={Board} preload={preloadBoard}/>
            </Route>
            <Route path="*404" component={NotFound} />
        </Router>
    );
}

export default Routing;