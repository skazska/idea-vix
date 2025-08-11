import { createForm, valiForm } from "@modular-forms/solid";
import { getError, getLabel, getString, getInput, type TFormComponent } from "../../common/gen/forms";
import { VerifyCodeSchema, type TVerifyCodeRequest, type TSessionData } from "../model";
import { Footer, Header } from "../../common/forms";
import { getSessionApi } from "../providers/api";
import { useBackend } from "../../common/providers/backend";
import { action, useAction } from "@solidjs/router";
import { createSignal } from "solid-js";

export const VerifyCodeForm: TFormComponent<TVerifyCodeRequest, TSessionData> = (props) => {
    const [_form, { Form, Field }] = createForm<TVerifyCodeRequest>({
        validate: valiForm(VerifyCodeSchema),
        initialValues: props.initialValues
    });

    const backend = useBackend();
    const sessionApi = getSessionApi(backend);

    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal<string | undefined>(undefined);

    const verifyCode = action(async (values: TVerifyCodeRequest) => {
        setLoading(true);
        setError(undefined);
        try {
            const sessionData = await sessionApi.verifyCode(values);
            props.onDone(sessionData);
            return sessionData;
        } catch (err) {
            setError("Invalid verification code. " + (err instanceof Error ? err.message : "Unknown error"));
        }
        setLoading(false);
    }, {});

    const onSubmit = useAction(verifyCode);

    return (
        <Form id="session-verify-form" onSubmit={onSubmit} onCancel={props.onCancel} class="space-y-4">
            <Header text={() => "Verify Code"} pending={loading}></Header>
            <div class="text-sm text-gray-600 mb-4">
                We've sent a verification code to <strong>{props.initialValues?.address}</strong>
            </div>
            <Field name="address">
                {(field, fieldProps) => (
                    <input {...fieldProps} type="hidden" value={field.value} />
                )}
            </Field>
            <Field name="code">
                {(field, props) => getInput(
                    getLabel({ name: field.name, label: "Verification Code", required: true }), 
                    getString(
                        { name: field.name, placeholder: "Enter verification code" }, 
                        field, 
                        props
                    ), 
                    getError(field)
                )}
            </Field>
            <Footer onCancel={props.onCancel} error={error}>
                Verify
            </Footer>
        </Form>
    )
}
