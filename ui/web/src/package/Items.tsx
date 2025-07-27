import { createEffect, For, type Component } from "solid-js";
import { usePackageData } from "./providers/items";
import { ListItemContainer, ListItemRowContainer } from "../common/list";
import { Edit } from "lucide-solid";
import { A } from "@solidjs/router";
import { ROUTE } from "./const";

const PackageItems: Component = () => {
    const [packageItems] = usePackageData();

    console.log("PackageItems rendered", packageItems.latest);
    createEffect(() => {
        console.log("PackageItems effect: packageItems changed", packageItems);
    });

    return (<>
        <For each={packageItems.latest}>{(item, _index) => {
                return (
                    <ListItemContainer type="row">
                        <ListItemRowContainer class="w-10 flex-none self-center">
                            <img src={item.icon} alt={item.name} />
                        </ListItemRowContainer>
                        <ListItemRowContainer class="w-26 flex-none">
                            <h3>{item.name}</h3>
                        </ListItemRowContainer>
                        <ListItemRowContainer class="flex-grow">
                            <p>{item.description}</p>
                        </ListItemRowContainer>
                        <ListItemRowContainer class="w-10 flex-none self-center">
                            <A href={`${ROUTE}/${item.id}`} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-m"> 
                                <Edit size={'1rem'}/>
                            </A>
                        </ListItemRowContainer>
                    </ListItemContainer>
                );
        }}</For>
    </>);
}

export default PackageItems;
