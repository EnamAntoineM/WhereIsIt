<script>
    import { logout, user } from '../stores/auth.js';
    
    // Function to log out
    const handleLogout = () => {
        logout();
    };
</script>

<main class="min-h-screen bg-white">
    <!--NAVBAR-->
    <section class="fixed top-0 left-0 h-16 w-full bg-primary text-black p-4 shadow-sm flex justify-between items-center rounded-2xl">
        <img src="/WIIT-GoldOnWhite.png" alt="WIIT Logo" class="ml-4 w-14">
        <button class ="bg-gray-100 border border-yellow-50 shadow-sm
            rounded-2xl hover:bg-gray-200 transition-colors duration-300
            px-4 py-2 flex items-center gap-2">
            <i class="fa-solid fa-circle-user"></i>
            <span class ="text-md font-bold"> Account </span>
        </button>
    </section>

    <!--LEFT PANEL-->
    <section class ="flex flex-col fixed top-16 left-0 w-64 h-[calc(100vh-4rem)]
        p-3 gap-3 items-center bg-gray-50 shadow-sm rounded-r-md border-r">
        <div class = "flex flex-col items-center gap-3 w-full">
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center text-center w-full gap-2">
                <i class="fas fa-box"></i>
                <span class ="text-md font-bold"> My items </span>
            </button>
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center text-center w-full gap-2">
                <i class="fas fa-chart-line"></i>
                <span class ="text-md font-bold"> Overview </span>
            </button>
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center text-center w-full gap-2">
                <i class="fas fa-file-import"></i>
                <span class ="text-md font-bold"> Import table </span>
            </button>
            <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center text-center w-full gap-2">
                <i class="fas fa-file-export"></i>
                <span class ="text-md font-bold"> Export table </span>
            </button>
        </div>
        <button class ="shadow-sm rounded-2xl hover:bg-gray-200 transition-colors duration-200
            p-5 flex items-center text-center w-full bottom-0 gap-2 mt-auto">
            <i class="fas fa-info-circle"></i>
            <span class ="text-md font-bold"> About us </span>
        </button>
    </section>
</main>