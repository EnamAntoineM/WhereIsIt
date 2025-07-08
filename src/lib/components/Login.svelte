<script>
    import { login } from "../stores/auth.js";
    import '../../app.css';
    let email = "";
    let password = "";
    let error = "";

    const handleLogin = async (e) => {
        e.preventDefault();
        error = "";

        if (!email || !password) {
            error = "Please enter both email and password";
            return;
        }

        if (email === "admin@example.com" && password === "admin") {
            login({ email, id: 1 });
        } else {
            error = "Invalid email or password";
        }
    };
</script>

<section class = "min-h-screen flex items-center justify-center bg-cover bg-center" style="background-image: url('/bg-login.jpg')">
        <div class="bg-white bg-opacity-90 rounded-2xl shadow-xl w-[500px] p-9 space-y-6">
            <div class="flex items-center justify-center gap-2">
                <h1 class="text-3xl font-bold text-yellow-900">Join Us</h1>
                <a href="https://github.com/EnamAntoineM" target="_blank">
                    <img src="/WIIT-GoldSymbol.png" alt="Logo" class="h-11 w-auto cursor-pointer" />
                </a>
            </div>

            {#if error}
                <div class="bg-red-100 text-red-700 border border-red-300 px-6 py-4 rounded-2xl flex items-center gap-2 text-md">
                    <i class="fas fa-exclamation-triangle"></i> {error}
                </div>
            {/if}

            <form on:submit={handleLogin} class="space-y-5">
                <div>
                    <label for="email" class="block mb-1 font-medium text-gray-700">
                        <i class="fas fa-user mr-2"></i>Email
                    </label>
                    <input
                            type="email"
                            id="email"
                            bind:value={email}
                            required
                            placeholder="Enter your email"
                            class="w-full border border-gray-400 rounded-2xl px-4 py-2 focus:outline-none focus:ring-2 focus:ring-yellow-200"
                    />
                </div>

                <div>
                    <label for="password" class="block mb-1 font-medium text-gray-700">
                        <i class="fas fa-key mr-2"></i>Password
                    </label>
                    <input
                            type="password"
                            id="password"
                            bind:value={password}
                            required
                            placeholder="Enter your password"
                            class="w-full border border-gray-400 rounded-2xl px-4 py-2 focus:outline-none focus:ring-2 focus:ring-yellow-200"
                    />
                </div>

                <button type="submit" class="w-full bg-yellow-600 hover:bg-yellow-700 text-white py-2 rounded-lg font-semibold">
                    Log In
                </button>

                <p class="text-center text-sm">
                    Don’t have an account?
                    <button class="text-yellow-700 hover:underline ml-1">
                        Sign up
                    </button>
                </p>
            </form>
        </div>
</section>