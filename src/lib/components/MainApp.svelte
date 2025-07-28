<script lang="ts">
    import { logout, user } from '../stores/auth.js';
    import { invoke } from '@tauri-apps/api/core';
    import {onMount} from "svelte";
    import { open } from '@tauri-apps/plugin-dialog';

    type Item = {
        name: string;
        units: number;
        registered_at: string;
        last_modified: string;
        expiry_date: string;
        location: string;
        note: string;
        visual: string;
        owner: string;
    };
    type MajItem = {
        id: number;
        name: string;
        units: number;
        registered_at: string;
        last_modified: string;
        expiry_date: string;
        location: string;
        note: string;
        visual: string;
        owner: string;
    }
    let showAddTableDialog = false;
    let showCreateItemDialog = false;
    let allowUpdate = false;
    let searched = "";
    let newTableName = "";
    let error_popup = "";
    let success_popup = "";
    let active_table: String = "bedroom";
    let items: MajItem[] = [];
    let newItem: Item = {
        name: "",
        units: 0,
        registered_at: "",
        last_modified: "",
        expiry_date: "",
        location: "",
        note: "",
        visual: "",
        owner: ""
    };
    let UpdatedItem: MajItem = {
        id: 0,
        name: "",
        units: 0,
        registered_at: "",
        last_modified: "",
        expiry_date: "",
        location: "",
        note: "",
        visual: "",
        owner: ""
    };
    onMount(() => {
      get_items();
    });
    async function handleUpdate(item_id: number) {
        if(!allowUpdate) {
            error_popup = "Nothing to save, please make changes before saving.";
            setTimeout(() => error_popup = "", 3000);
            return;
        }
        try {
            UpdatedItem.last_modified = new Date().toISOString().split('T')[0];
            console.log(await invoke('update_item_cmd', {item: UpdatedItem, itemId: item_id, currentTable: active_table}));
            allowUpdate = false;
            success_popup = "Item(s) updated successfully !";
            setTimeout(() => success_popup = "", 2000);
            await get_items();
            UpdatedItem = {
                id: 0,
                name: "",
                units: 0,
                registered_at: "",
                last_modified: "",
                expiry_date: "",
                location: "",
                note: "",
                visual: "",
                owner: ""
            };
        } catch(error) {
            error_popup = "Failed to update item: " + error;
            setTimeout(() => error_popup = "", 3000);
            return;
        }
    }
    async function set_base_settings(item: Item) {
        item.last_modified = new Date().toISOString().split('T')[0];
        item.registered_at = new Date().toISOString().split('T')[0];
        item.owner = "enam";
    }
    async function handleSearch() {
        if (!searched.trim()) {
            await get_items();
            return;
        }
        try {
            items = await invoke<MajItem[]>('search_element_cmd', {element: searched, tableName: active_table});
        } catch (error) {
            error_popup = "Failed to search items: " + error;
            setTimeout(() => error_popup = "", 3000);
        }
    }
    async function handleCreateTable() {
        if (!newTableName.trim()) {
            error_popup = "Please enter a table name.";
            setTimeout(() => error_popup = "", 3000);
            return;
        }
        try {
            await invoke('create_table_cmd', { tableName: newTableName.trim() });
            success_popup = `✅ Table '${newTableName.trim()}' created successfully`;
            setTimeout(() => success_popup = "", 2000);
            showAddTableDialog = false;
            newTableName = "";
        } catch (error) {
            error_popup = "❌ Failed to create table: " + error;
            setTimeout(() => error_popup = "", 3000);
            console.error(error);
        }
    }
    async function handleAddItem() {
        if (!newItem.name.trim()) {
            error_popup = "Please enter a name for your item.";
            setTimeout(() => error_popup = "", 3000);

            return;
        }
        try {
            await invoke('add_item_cmd', { item: newItem, currentTable: active_table});
            success_popup = "Item added successfully!";
            setTimeout(() => success_popup = "", 2000);
            await get_items();
            showCreateItemDialog = false;
            newItem = {
                name: "",
                units: 0,
                registered_at: "",
                last_modified: "",
                expiry_date: "",
                location: "",
                note: "",
                visual: "",
                owner: ""
            }
            await set_base_settings(newItem);
        } catch (error) {
            error_popup = "Failed to add item: " + error;
            setTimeout(() => error_popup = "", 3000);
        }
    }
    async function get_items() {
        try {
            items = await invoke<MajItem[]>('get_all_items_cmd', { table: active_table });
        } catch (error) {
            error_popup = "Failed to fetch items:" + error;
            setTimeout(() => error_popup = "", 3000);
        }
    }
    async function selectImage() {
        const selected = await open({
            multiple: false,
            filters: [
                { name: 'Image', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp'] }
            ]
        });

        if (typeof selected === 'string') {
            newItem.visual = selected;  // Save image path to your visual field
        }
    }
    async function handleDeleteItem(item_id: number) {
        try {
            success_popup = await invoke('delete_item_cmd', {itemId: item_id, currentTable: active_table});
            setTimeout(() => success_popup = "", 2000);
            await get_items();
        } catch (error) {
            error_popup = "Failed to delete items: " + error;
            setTimeout(() => error_popup = "", 3000);
        }
    }
    async function handleDeleteTable(tableName: string) {
        try {
            success_popup = await invoke('delete_table_cmd', {tableName: tableName});
            setTimeout(() => success_popup = "", 2000);
        } catch (error) {
            error_popup = "Failed to delete table: " + error;
            setTimeout(() => error_popup = "", 3000);
        }
    }

    function cancelDialog() {
        showAddTableDialog = false;
        newTableName = "";
        success_popup = "";
        error_popup = "";  // ✅ clear old error
    }
    function cancelNewItemDialog() {
        showCreateItemDialog = false;
        newItem = {
            name: "",
            units: 0,
            registered_at: "",
            last_modified: "",
            expiry_date: "",
            location: "",
            note: "",
            visual: "",
            owner: ""
        };
        error_popup = "";
        success_popup = "";// ✅ clear old error
    }
    const handleLogout = () => {
        logout();
    };
    const saveChanges = () => {
        console.log("Saved items: ", items);
    };
    // Which fields the user wants to show
    let selectedFields = {
        name: true,
        units: true,
        registeredAt: true,
        expiryDate: true,
        lastModified: true,
        location: true,
        note: true,
        visual: true
    };
</script>

<main class="min-h-screen bg-white">
    <!--NAVBAR-->
    <section class="fixed top-0 left-0 h-16 w-full bg-primary text-black p-4
        flex justify-between border-b items-center rounded-2xl">
        <img src="/WIIT-GoldOnWhite.png" alt="WIIT Logo" class="ml-4 w-14">
        <button class ="bg-gray-100 border border-yellow-50 shadow-sm
            rounded-2xl hover:bg-gray-200 transition-colors duration-300
            px-4 py-2 flex items-center gap-2">
            <i class="fa-solid fa-circle-user"></i>
            <span class ="text-md font-bold "> Account </span>
        </button>
    </section>

    <!--LEFT PANEL-->
    <section class ="flex flex-col fixed top-16 left-0 w-64 h-[calc(100vh-4rem)]
        p-3 gap-3 items-center bg-gray-50 shadow-sm rounded-r-md border-r">
        <div class = "flex flex-col items-center gap-3 w-full">
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center w-full gap-2">
                <i class="fas fa-box"></i>
                <span class ="text-md font-bold"> My items </span>
            </button>
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center w-full gap-2" on:click={handleLogout}>
                <i class="fas fa-chart-line"></i>
                <span class ="text-md font-bold"> Overview </span>
            </button>
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center w-full gap-2" on:click={handleLogout}>
                <i class="fas fa-ellipsis-h"></i>
                <span class ="text-md font-bold"> Other items </span>
            </button>
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center w-full gap-2">
                <i class="fas fa-file-import"></i>
                <span class ="text-md font-bold"> Import table </span>
            </button>
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center w-full gap-2">
                <i class="fas fa-file-export"></i>
                <span class ="text-md font-bold"> Export table </span>
            </button>
        </div>
        <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center w-full bottom-0 gap-2 mt-auto">
            <i class="fas fa-info-circle"></i>
            <span class ="text-md font-bold"> About us </span>
        </button>
    </section>

    <!--MAIN CONTENT-->
    <section class="pt-16 ml-64 flex flex-col min-h-screen">
        <!-- Top bar -->
        <div class="flex shadow-sm justify-between items-center p-8 pl-8 border-b h-16 rounded-md bg-white">
            <h1 class="text-2xl font-semibold"> My Items</h1>
            <div class="flex justify-between space-x-2 h-12">
                <button on:click={() => showCreateItemDialog = true} class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
                   px-5 flex items-center justify-center gap-2">
                    <i class="fa-solid fa-plus"></i>
                    <span class="text-md font-bold"> Add Item </span>
                </button>
                <button class="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
                   px-5 flex items-center justify-center gap-2">
                    <i class="fa-solid fa-filter"></i>
                    <span class ="text-md font-bold"> Filter </span>
                </button>
                <input type="text" bind:value={searched} on:input = {handleSearch} placeholder="Search..." class="border rounded-2xl
                    px-4 py-2 focus:outline-none focus:ring-2 focus:ring-yellow-300" />
            </div>
        </div>

        <div class="flex-1 p-4 pb-24">
            <!-- Table or empty state -->
            {#if items.length === 0}
                <p class="text-center text-gray-500 italic mt-8">
                    No items yet. Start by adding one.
                </p>
            {:else}
                <div class="overflow-x-auto overflow-y-auto shadow-md rounded-lg">
                    <table class="min-w-full text-sm text-left">
                        <thead class="bg-yellow-300 text-gray-800 uppercase text-xs">
                        <tr>
                            {#if selectedFields.name}
                                <th class="px-6 py-3">Name</th>
                            {/if}
                            {#if selectedFields.units}
                                <th class="px-6 py-3">Units</th>
                            {/if}
                            {#if selectedFields.registeredAt}
                                <th class="px-6 py-3">Registered At</th>
                            {/if}
                            {#if selectedFields.expiryDate}
                                <th class="px-6 py-3">Expiry Date</th>
                            {/if}
                            {#if selectedFields.lastModified}
                                <th class="px-6 py-3">Last modified</th>
                            {/if}
                            {#if selectedFields.location}
                                <th class="px-8 py-3">Location</th>
                            {/if}
                            {#if selectedFields.note}
                                <th class="px-8 py-3">Note</th>
                            {/if}
                            {#if selectedFields.visual}
                                <th class="px-4 py-3">Visual</th>
                            {/if}
                            <th class="px-4 py-3"> </th>
                        </tr>
                        </thead>
                        <tbody>
                        {#each items as item}
                            <tr class="border-t transition-all duration-200 focus:outline-none rounded-2xl focus:ring-1 focus:ring-blue-300 focus:border focus:border-blue-400" tabindex="0">
                                {#if selectedFields.name}
                                    <td class="px-2 py-2">
                                        <input type="text" on:focus={() => UpdatedItem = item} on:input={() => allowUpdate = true} class = "p-2 rounded-xl hover:bg-gray-100 px-4 py-2
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.name} />
                                    </td>
                                {/if}
                                {#if selectedFields.units}
                                    <td class="px-6 py-2">
                                        <input type="number" on:focus={() => UpdatedItem = item} on:input={() => allowUpdate = true} class = "p-2 rounded-xl max-w-10 text-center hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.units} />
                                    </td>
                                {/if}
                                {#if selectedFields.registeredAt}
                                    <td class="px-4 py-2">
                                        <input type="date" on:focus={() => UpdatedItem = item} on:input={() => allowUpdate = true} class = "p-2 rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.registered_at} />
                                    </td>
                                {/if}
                                {#if selectedFields.expiryDate}
                                    <td class="px-4 py-2">
                                        <input type="date" on:focus={() => UpdatedItem = item} on:input={() => allowUpdate = true} class = "p-2 rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.expiry_date} />
                                    </td>
                                {/if}
                                {#if selectedFields.lastModified}
                                    <td class="px-4 py-2">
                                        <input type="date" on:focus={() => UpdatedItem = item} on:input={() => allowUpdate = true} class = "p-2 rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.last_modified} />
                                    </td>
                                {/if}
                                {#if selectedFields.location}
                                    <td class="px-4 py-2">
                                        <input type="text" on:focus={() => UpdatedItem = item} on:input={() => allowUpdate = true} class = "p-2 px-4  rounded-xl max-w-36 hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.location} />
                                    </td>
                                {/if}
                                {#if selectedFields.note}
                                    <td class="px-4 py-2">
                                        <input type="text" on:focus={() => UpdatedItem = item} on:input={() => allowUpdate = true} class = "p-2 px-4  rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.note} />
                                    </td>
                                {/if}
                                {#if selectedFields.visual}
                                    <td class="px-6 py-2">
                                        {#if item.visual}
                                            <img src={`file://${item.visual}`} alt="Preview" class="w-12 h-12 object-cover cursor-pointer rounded mt-2" />
                                        {:else}
                                            -
                                        {/if}
                                    </td>
                                {/if}
                                <td class="px-4 py-2 space-x-2">
                                    <!-- svelte-ignore a11y_consider_explicit_label -->
                                    <button on:click={() => handleUpdate(item.id)} class = "px-4 py-2 rounded-xl bg-green-300 hover:bg-green-400
                                        transition-colors duration-200 focus:outline-none">
                                        <i class="fa-solid fa-save"></i>
                                    </button>
                                    <!-- svelte-ignore a11y_consider_explicit_label -->
                                    <button on:click={() => handleDeleteItem(item.id)} class = "px-4 py-2 rounded-xl bg-red-300 hover:bg-red-400
                                        transition-colors duration-200 focus:outline-none">
                                        <i class="fa-solid fa-trash"></i>
                                    </button>
                                </td>
                            </tr>
                        {/each}
                        </tbody>
                    </table>
                </div>
            {/if}
        </div>
    </section>

    <!--Bottom bar-->
    <section class="flex fixed bottom-0 items-center justify-end left-64 right-0 z-10 bg-white gap-2 p-6 h-16 border-t shadow-sm rounded-sm">
        <button class="shadow-sm rounded-2xl bg-red-100 hover:bg-red-300 transition-colors duration-200
                   p-4 flex items-center justify-center gap-2 ml-auto w-full ">
            <i class="fa-solid fa-trash"></i>
            <span class="text-md font-bold"> Delete table </span>
        </button>
        <button on:click={() => showAddTableDialog = true} class="shadow-sm rounded-2xl bg-gray-50 hover:bg-gray-200 transition-colors duration-200
                   p-4 flex justify-center items-center text-center gap-2 ml-auto w-full ">
            <i class="fa-solid fa-plus"></i>
            <span class="text-md font-bold"> Add table </span>
        </button>
        <button class="shadow-sm rounded-2xl bg-gray-50 hover:bg-gray-200 transition-colors duration-200
                   p-4 flex items-center justify-center gap-2 ml-auto w-full">
            <i class="fa-solid fa-right-left"></i>
            <span class="text-md font-bold"> Switch table </span>
        </button>
    </section>

    {#if showCreateItemDialog}
        <div class="fixed inset-0 bg-white bg-opacity-40 z-50 flex flex-col items-center shadow-2xl p-14 justify-center">
            <div class="bg-white p-6 rounded-2xl shadow-2xl w-full max-w-xl border border-gray-300 space-y-2">
                <h2 class="text-xl ml-4 mr-4 font-semibold">Create a new item</h2>
                <div class="flex flex-col gap-3 bg-white p-6 rounded-2xl w-full space-y-4">
                    <input type="text" required bind:value={newItem.name}
                           placeholder="Enter item name"
                           class="w-full p-2 border rounded-xl focus:outline-none focus:ring-2 focus:ring-yellow-300" />
                    <input type="number" bind:value={newItem.units}
                           placeholder="Units..."
                           class="w-full p-2 border rounded-xl focus:outline-none focus:ring-2 focus:ring-yellow-300" />
                    <input type="date" bind:value={newItem.expiry_date}
                           placeholder="Date expiry..."
                           class="w-full p-2 border rounded-xl focus:outline-none focus:ring-2 focus:ring-yellow-300" />
                    <input type="text" bind:value={newItem.location}
                           placeholder="Location..."
                           class="w-full p-2 border rounded-xl focus:outline-none focus:ring-2 focus:ring-yellow-300" />
                    <input type="text" bind:value={newItem.note}
                           placeholder="Any information for later..."
                           class="w-full p-2 border rounded-xl focus:outline-none focus:ring-2 focus:ring-yellow-300" />
                    <div class="flex items-center gap-2">
                        <button on:click={selectImage}
                                class="px-4 py-2 rounded-xl bg-blue-100 hover:bg-blue-200">
                            Choose Image
                        </button>
                        {#if newItem.visual}
                            <span class="text-sm text-gray-600 truncate max-w-[200px]">{newItem.visual}</span>
                        {/if}
                    </div>

                    <div class="flex justify-end gap-2">
                        <button on:click= {cancelNewItemDialog}
                                class="px-4 py-2 rounded-xl bg-gray-100 hover:bg-gray-200">
                            Cancel
                        </button>
                        <button on:click={handleAddItem}
                                class="px-4 py-2 rounded-xl bg-yellow-300 hover:bg-yellow-400 font-bold">
                            Create
                        </button>
                    </div>
                </div>
            </div>

        </div>
    {/if}
    {#if showAddTableDialog}
        <div class="fixed inset-0 bg-black bg-opacity-40 z-50 flex items-center justify-center">
            <div class="bg-white p-6 rounded-2xl shadow-xl w-full max-w-md space-y-4">
                <h2 class="text-xl font-semibold">Create a New Table</h2>
                <input type="text" bind:value={newTableName}
                       placeholder="Enter table name"
                       class="w-full p-2 border rounded-xl focus:outline-none focus:ring-2 focus:ring-yellow-300" />

                <div class="flex justify-end gap-2">
                    <button on:click= {cancelDialog}
                            class="px-4 py-2 rounded-xl bg-gray-100 hover:bg-gray-200">
                        Cancel
                    </button>
                    <button on:click={handleCreateTable}
                            class="px-4 py-2 rounded-xl bg-yellow-300 hover:bg-yellow-400 font-bold">
                        Create
                    </button>
                </div>
            </div>
        </div>
    {/if}
    {#if error_popup}
        <div class="fixed top-6 right-6 z-50 px-5 py-4 bg-red-50 border border-red-700 text-black text-md rounded-2xl shadow-sm">
            {error_popup}
        </div>
    {/if}
    {#if success_popup}
        <div class="fixed top-6 right-6 z-50 px-5 py-4 bg-green-50 border border-green-700 text-black text-md rounded-2xl shadow-sm">
            {success_popup}
        </div>
    {/if}
</main>