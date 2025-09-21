import { createForm, valiForm } from "@modular-forms/solid";
import { getError, getLabel, getInput, getString, getText, type TFormComponent, getBoolean } from "../../common/form/generators";
import { NewPackageSchema, type TPackage, type TPackageNew } from "../model";
import { Footer, Header } from "../../common/form/components";
import { usePackageData } from "../providers/items";
import { action, useAction } from "@solidjs/router";
import { createSignal } from "solid-js";
import { ENTITY_NAME } from "../const";

export const NewPackageForm: TFormComponent<TPackageNew, TPackage> = (props) => {
    const [_form, { Form, Field }] = createForm<TPackageNew>({
        validate: valiForm(NewPackageSchema),
        initialValues: props.initialValues
    });

    const [, actions] = usePackageData();

    const [loading, setLoading] = createSignal(false);
    const [error, setError] = createSignal<string | undefined>(undefined);

    const add = action(async (values: TPackageNew) => {
        setLoading(true);
        setError(undefined);
        try {
            const item = await actions.add(values);
            props.onDone(item);
            return item;
        } catch (err) {
            setError(`Failed to add ${ENTITY_NAME}. ${err instanceof Error ? err.message : "Unknown error"}`);
        }
        setLoading(false);
    }, {});

    // const submission = useSubmission(add);
    const onSubmit = useAction(add);

    return (<Form onSubmit={onSubmit} onCancel={props.onCancel} class="space-y-4" data-testid="package-new-form">
        <Header text={() => `New ${ENTITY_NAME}`} pending={loading}></Header>
        <Field name="name">
            {(field, props) => getInput(getLabel(field), getString(field, field, props), getError(field))}
        </Field>
        <Field name="slug">
            {(field, props) => getInput(getLabel(field), getString(field, field, props), getError(field))}
        </Field>
        <Field name="description" >
            {(field, props) => getInput(getLabel(field), getText(field, field, props), getError(field))}
        </Field>
        <Field name="icon">
            {(field, props) => getInput(getLabel(field), getString(field, field, props), getError(field))}
        </Field>
        <Field name="is_public" type="boolean">
            {(field, props) => getInput(getBoolean({ ...field, label: 'Public' }, field, props), getError(field))}
        </Field>
        <Footer onCancel={props.onCancel} error={error}>
        </Footer>
    </Form>)
}
