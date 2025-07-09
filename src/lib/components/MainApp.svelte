<script>
    import { logout, user } from '../stores/auth.js';

    // Function to log out
    const handleLogout = () => {
        logout();
    };
    // Sample items
    let items = [
        {
            name: "Stapler",
            units: 2,
            registeredAt: "2025-06-12",
            expiryDate: "2026-06-12",
            location: "Drawer A",
            note: "Bought in bulk",
            visual: "/images/stapler.jpg"
        },
        {
            name: "Notebook",
            units: 5,
            registeredAt: "2025-07-01",
            expiryDate: "",
            location: "Shelf B",
            note: "",
            visual: ""
        }
    ];

    // Which fields the user wants to show
    let selectedFields = {
        name: true,
        units: true,
        registeredAt: true,
        expiryDate: true,
        location: true,
        note: false,
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
                <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
                   px-5 flex items-center justify-center gap-2">
                    <i class="fa-solid fa-plus"></i>
                    <span class="text-md font-bold"> Add Item </span>
                </button>
                <button class="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
                   px-5 flex items-center justify-center gap-2">
                    <i class="fa-solid fa-filter"></i>
                    <span class ="text-md font-bold"> Filter </span>
                </button>
                <input type="text" placeholder="Search..." class="border rounded-2xl
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
                        <thead class="bg-yellow-100 text-gray-800 uppercase text-xs">
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
                            <tr class="border-t hover:bg-yellow-50">
                                {#if selectedFields.name}
                                    <td class="px-4 py-2">{item.name}</td>
                                {/if}
                                {#if selectedFields.units}
                                    <td class="px-4 py-2">{item.units}</td>
                                {/if}
                                {#if selectedFields.registeredAt}
                                    <td class="px-4 py-2">{item.registeredAt}</td>
                                {/if}
                                {#if selectedFields.expiryDate}
                                    <td class="px-4 py-2">{item.expiryDate || '-'}</td>
                                {/if}
                                {#if selectedFields.location}
                                    <td class="px-4 py-2">{item.location}</td>
                                {/if}
                                {#if selectedFields.note}
                                    <td class="px-4 py-2">{item.note || '-'}</td>
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
        <button class="shadow-sm rounded-2xl bg-gray-50 hover:bg-gray-200 transition-colors duration-200
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
</main>