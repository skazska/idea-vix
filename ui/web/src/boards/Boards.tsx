// SolidJs typescript component for the board application

import { ErrorBoundary } from "solid-js";
import { setTitle } from "../common/providers/page-state";
import { BoardsProvider } from "./providers/boards";
import BoardsItems from "./Items";

export default function Boards() {
  setTitle("Boards");

  return (
    <div class="p-4">
      <h2>Welcome to the Board Application</h2>
      <ErrorBoundary fallback={(error) => (
          <div class="error">
              <h2>Error loading board list</h2>
              <p>{error.message}</p>
              <p>{error.stack}</p>
          </div>
      )}>
          <BoardsProvider>
              <BoardsItems />
          </BoardsProvider>
      </ErrorBoundary>
    </div>
  );
}