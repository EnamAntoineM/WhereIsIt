<script>
    import { logout, user } from '../stores/auth.js';

    // Function to log out
    const handleLogout = () => {
        logout();
    };
</script>

<div class="main-app">
    <header class="app-header">
        <h1>Welcome to your Application!</h1>
        <div class="user-info">
            <span>Hello, {$user?.email || 'User'}!</span>
            <button class="logout-btn" on:click={handleLogout}>
                <i class="fas fa-sign-out-alt"></i>
                Logout
            </button>
        </div>
    </header>

    <main class="app-content">
        <div class="dashboard">
            <h2>Dashboard</h2>
            <p>This is your main application content!</p>
            <p>You can build your actual app features here.</p>
        </div>
    </main>
</div>

<style>
    .main-app {
        min-height: 100vh;
        background-color: #f5f5f5;
    }

    .app-header {
        background-color: #4891ff;
        color: white;
        padding: 1rem 2rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }

    .app-header h1 {
        margin: 0;
    }

    .user-info {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .logout-btn {
        background-color: #dc3545;
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 8px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        transition: background-color 0.3s;
    }

    .logout-btn:hover {
        background-color: #c82333;
    }

    .app-content {
        padding: 2rem;
    }

    .dashboard {
        background-color: white;
        padding: 2rem;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }

    .dashboard h2 {
        color: #333;
        margin-bottom: 1rem;
    }

    .dashboard p {
        color: #666;
        margin-bottom: 1rem;
    }
</style>
