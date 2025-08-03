import { usePageState } from "../common/providers/page-state";
import { Header, Close } from "../common/forms";

export default function SessionInfo(props: { onClose: () => void }) {
  const [pageState, setPageState] = usePageState();

  const handleSignOut = () => {
    setPageState.setSessionAddress(undefined);
    setPageState.setSessionToken(undefined);
    props.onClose();
  };

  return (
    <div class="space-y-4">
      <Header text={() => "Session Info"}>
      </Header>
      
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">
          Session Address
        </label>
        <div class="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50">
          {pageState.sessionAddress()}
        </div>
      </div>

      

      <div class="flex items-center justify-between pt-4 md:pt-5 border-t rounded-b dark:border-gray-600 border-gray-200">
        <Close onClick={props.onClose}>Close</Close>
        <button
          type="button"
          class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
          onClick={handleSignOut}
        >
          Sign Out
        </button>
      </div>
    </div>
  );
}
