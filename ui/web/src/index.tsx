/**
 * Index page
 * @file ui/web/src/index.tsx
 * @author ska
 * 
 * This file is the entry point for the SolidJS application.
 * 
 * It sets up the router, defines routes, and renders the application.
 */

import { render } from 'solid-js/web'
import './index.css'
import { PageStateProvider } from './common/providers/page-state.tsx'
import { BackendProvider } from './common/providers/backend.tsx'
import Routing from './Routing.tsx'

const root = document.getElementById('root')



render(() => (
    <PageStateProvider title='Shapes' userId='' sessionAddress={undefined} sessionToken={undefined}>
        <BackendProvider>
            <Routing />
        </BackendProvider>
    </PageStateProvider>
), root!)
