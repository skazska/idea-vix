import { createEffect, For, type Component } from "solid-js";
import { usePackageData } from "./providers/items";
import { ListItemContainer, ListItemRowContainer } from "../common/list/components";
import { Edit } from "lucide-solid";
import { A } from "@solidjs/router";
import { ROUTE } from "./const";

const PackageItems: Component = () => {
    const [packageItems] = usePackageData();

    console.log("PackageItems rendered", packageItems.latest);
    createEffect(() => {
        console.log("PackageItems effect: packageItems changed", packageItems);
    });

    return (<div role="list" data-testid="package-items">
        <For each={packageItems.latest}>{(item, _index) => {
                return (
                    <ListItemContainer type="row" key={() => item.id}>
                        <ListItemRowContainer class="w-10 flex-none self-center">
                            <img src={item.icon} alt={item.name} data-testid="package-item-icon" />
                        </ListItemRowContainer>
                        <ListItemRowContainer class="w-26 flex-none">
                            <h3 class="flex items-center gap-2">
                                <span data-testid="package-item-name">{item.name}</span>
                                <span data-testid="package-item-visibility" class={`text-xxs px-1 py-0.5 rounded ${item.is_public ? 'bg-green-100 text-green-700' : 'bg-gray-200 text-gray-700'}`}>
                                    {item.is_public ? 'Public' : 'Private'}
                                </span>
                            </h3>
                        </ListItemRowContainer>
                        <ListItemRowContainer class="flex-grow">
                            <p data-testid="package-item-description">{item.description}</p>
                        </ListItemRowContainer>
                        <ListItemRowContainer class="w-10 flex-none self-center">
                            <A data-testid="open-package-item" href={`${ROUTE}/${item.id}`} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-m">
                                <Edit size={'1rem'}/>
                            </A>
                        </ListItemRowContainer>
                    </ListItemContainer>
                );
        }}</For>
    </div>);
}

export default PackageItems;
