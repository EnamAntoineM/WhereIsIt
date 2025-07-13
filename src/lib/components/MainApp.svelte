<script lang="ts">
    import { logout, user } from '../stores/auth.js';
    import { invoke } from '@tauri-apps/api/core';
    import {onMount} from "svelte";

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
    let showAddTableDialog = false;
    let showCreateItemDialog = false;
    let searched = "";
    let newTableName = "";
    let error_popup = "";
    let success_popup = "";
    let items: Item[] = [];
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
    onMount(() => {
      get_items();
    });
    async function set_base_settings() {
        newItem.registered_at = new Date().toISOString().split('T')[0];
        newItem.owner = "enam";
    }
    async function handleSearch() {
        if (!searched.trim()) {
            return;
        }
        try {
            items = await invoke<Item[]>('search_element_cmd', {element: searched, tableName: "kitchen"});
        } catch (error) {
            error_popup = "Failed to search items: " + error;
        }
    }
    async function handleCreateTable() {
        if (!newTableName.trim()) {
            error_popup = "Please enter a table name.";
            return;
        }
        try {
            await invoke('create_table_cmd', { tableName: newTableName.trim() });
            success_popup = `✅ Table '${newTableName.trim()}' created successfully`;
            showAddTableDialog = false;
            newTableName = "";
        } catch (error) {
            error_popup = "❌ Failed to create table: " + error;
            console.error(error);
        }
    }
    async function handleAddItem() {
        try {
            await invoke('add_item_cmd', { item: newItem, currentTable: "kitchen"});
            success_popup = "Item added successfully!";
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
            await set_base_settings();
        } catch (error) {
            error_popup = "Failed to add item: " + error;
        }
    }
    async function get_items() {
        try {
            items = await invoke<Item[]>('get_all_items_cmd', { table: "kitchen" });
        } catch (error) {
            error_popup = "Failed to fetch items:" + error;
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
            <h1 class="text-2xl font-semibold">My Items</h1>
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
                                <th class="px-4 py-3">Name</th>
                            {/if}
                            {#if selectedFields.units}
                                <th class="px-4 py-3">Units</th>
                            {/if}
                            {#if selectedFields.registeredAt}
                                <th class="px-4 py-3">Registered At</th>
                            {/if}
                            {#if selectedFields.expiryDate}
                                <th class="px-4 py-3">Expiry Date</th>
                            {/if}
                            {#if selectedFields.location}
                                <th class="px-4 py-3">Location</th>
                            {/if}
                            {#if selectedFields.note}
                                <th class="px-4 py-3">Note</th>
                            {/if}
                            {#if selectedFields.visual}
                                <th class="px-4 py-3">Visual</th>
                            {/if}
                        </tr>
                        </thead>
                        <tbody>
                        {#each items as item}
                            <tr class="border-t">
                                {#if selectedFields.name}
                                    <td class="px-4 py-2">
                                        <input type="text" class = "p-2 rounded-xl hover:bg-gray-100 px-4 py-2
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.name} />
                                    </td>
                                {/if}
                                {#if selectedFields.units}
                                    <td class="px-4 py-2">
                                        <input type="text" class = "p-2 rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.units} />
                                    </td>
                                {/if}
                                {#if selectedFields.registeredAt}
                                    <td class="px-4 py-2">
                                        <input type="date" class = "p-2 rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.registered_at} />
                                    </td>
                                {/if}
                                {#if selectedFields.expiryDate}
                                    <td class="px-4 py-2">
                                        <input type="date" class = "p-2 rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.expiry_date} />
                                    </td>
                                {/if}
                                {#if selectedFields.location}
                                    <td class="px-4 py-2">
                                        <input type="text" class = "p-2 rounded-xl hover:bg-gray-100
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.location} />
                                    </td>
                                {/if}
                                {#if selectedFields.note}
                                    <td class="px-4 py-2">
                                        <input type="text" class = "p-2 rounded-xl hover:bg-gray-100 
                                            transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-yellow-300" bind:value={item.note} />
                                    </td>
                                {/if}
                                {#if selectedFields.visual}
                                    <td class="px-4 py-2">
                                        {#if item.visual}
                                            <img src={item.visual} alt="Item" class="w-10 h-10 object-cover rounded"/>
                                        {:else}
                                            -
                                        {/if}
                                    </td>
                                {/if}
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
            <i class="fa-solid fa-save"></i>
            <span class="text-md font-bold"> Save changes </span>
        </button>
    </section>

    {#if showCreateItemDialog}
        <div class="fixed inset-0 bg-white bg-opacity-40 z-50 flex flex-col items-center shadow-xl border-4 border-gray-400 p-14 justify-center">
            <h2 class="text-xl font-semibold shadow-md">Create a new item</h2>
            <div class="flex gap-3 bg-white p-6 rounded-2xl shadow-xl w-full space-y-4">
                <input type="text" bind:value={newItem.name}
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
                <input type="text" bind:value={newItem.visual}
                       placeholder="Path of the image..."
                       class="w-full p-2 border rounded-xl focus:outline-none focus:ring-2 focus:ring-yellow-300" />

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