import { createSignal, Show } from "solid-js";
import { User, UserCheck } from "lucide-solid";
import { usePageState } from "../providers/page-state";
import { ModalCentered } from "../modals";
import { SignInForm } from "./forms/signin";
import { VerifyCodeForm } from "./forms/verify";
import SessionInfo from "./SessionInfo";
import type { TSessionData } from "./model";

export default function SessionIcon() {
  const [pageState, setPageState] = usePageState();
  const [showModal, setShowModal] = createSignal(false);
  const [step, setStep] = createSignal<'signin' | 'verify' | 'info'>('signin');
  const [address, setAddress] = createSignal('');

  // Now we only check for the address since the token is securely stored in an HttpOnly cookie
  const isSignedIn = () => !!pageState.sessionAddress();

  const handleIconClick = () => {
    if (isSignedIn()) {
      setStep('info');
    } else {
      setStep('signin');
    }
    setShowModal(true);
  };

  const handleSignInDone = (result: { step: 'verify', address: string } | undefined) => {
    if (result) {
      setAddress(result.address);
      setStep('verify');
    }
  };

  const handleVerifyDone = (sessionData: TSessionData | undefined) => {
    if (sessionData) {
      setPageState.setSessionAddress(sessionData.address);
      setPageState.setSessionOk(true);
      setPageState.setSessionExpiresAt(sessionData.expires_at);
      setShowModal(false);
    }
  };

  const handleCancel = () => {
    setShowModal(false);
    setStep('signin');
    setAddress('');
  };

  return (
    <>
      <button
        onClick={handleIconClick}
        class="p-2 rounded hover:bg-gray-700 transition-colors"
        title={isSignedIn() ? `Signed in as ${pageState.sessionAddress()}` : "Sign in"}
        data-testid="session-icon"
      >
        {isSignedIn() ? (
          <UserCheck size={20} class="text-green-400" />
        ) : (
          <User size={20} class="text-gray-400" />
        )}
      </button>

      <Show when={showModal()}>
        <ModalCentered>
          <Show when={step() === 'signin'}>
            <SignInForm 
              onDone={handleSignInDone}
              onCancel={handleCancel}
            />
          </Show>
          
          <Show when={step() === 'verify'}>
            <VerifyCodeForm 
              initialValues={{ address: address(), code: '' }}
              onDone={handleVerifyDone}
              onCancel={handleCancel}
            />
          </Show>
          
          <Show when={step() === 'info'}>
            <SessionInfo onClose={handleCancel} />
          </Show>
        </ModalCentered>
      </Show>
    </>
  );
}
