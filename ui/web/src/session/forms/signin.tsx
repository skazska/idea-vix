import { createForm, valiForm } from "@modular-forms/solid";
import { getError, getLabel, getString, getInput, type TFormComponent } from "../../common/gen/forms";
import { SignInSchema, type TSignInRequest } from "../model";
import { Footer, Header } from "../../common/forms";
import { getSessionApi } from "../providers/api";
import { useBackend } from "../../common/providers/backend";
import { action, useAction } from "@solidjs/router";
import { createSignal } from "solid-js";

export const SignInForm: TFormComponent<TSignInRequest, { step: 'verify', address: string }> = (props) => {
    const [_form, { Form, Field }] = createForm<TSignInRequest>({
        validate: valiForm(SignInSchema),
        initialValues: props.initialValues
    });

    const backend = useBackend();
    const sessionApi = getSessionApi(backend);

    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal<string | undefined>(undefined);

    const signIn = action(async (values: TSignInRequest) => {
        setLoading(true);
        setError(undefined);
        try {
            await sessionApi.signIn(values);
            props.onDone({ step: 'verify', address: values.address });
            return { step: 'verify', address: values.address };
        } catch (err) {
            setError("Failed to send verification code. " + (err instanceof Error ? err.message : "Unknown error"));
        }
        setLoading(false);
    }, {});

    const onSubmit = useAction(signIn);

    return (
        <Form onSubmit={onSubmit} onCancel={props.onCancel} class="space-y-4" data-testid="session-signin-form">
            <Header text={() => "Sign In"} pending={loading}></Header>
            <Field name="address">
                {(field, props) => getInput(
                    getLabel({ name: field.name, label: "Email Address", required: true }), 
                    getString(
                        { name: field.name, placeholder: "Enter your email address" }, 
                        field, 
                        props
                    ), 
                    getError(field)
                )}
            </Field>
            <Footer onCancel={props.onCancel} error={error}></Footer>
        </Form>
    )
}
