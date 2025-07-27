// Solidjs typescript App for the home page 

import { APP_NAME } from "../common/const";
import { setTitle } from "../common/providers/page-state";

export default function App() {
    setTitle(APP_NAME);

    return (
        <div class="p-4">
            <h2>Welcome to the Home Page</h2>
            <p>This is a simple SolidJS application.</p>
            <p>Explore the features and learn more about SolidJS.</p>
        </div>
    );
}