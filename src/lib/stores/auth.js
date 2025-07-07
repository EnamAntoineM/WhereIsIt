import { writable } from 'svelte/store';

// Create memory that remembers login status
export const isAuthenticated = writable(false);
export const user = writable(null);

// Function to log someone in
export const login = (userData) => {
    isAuthenticated.set(true);
    user.set(userData);
    // Save to browser storage so it remembers after closing app
    localStorage.setItem('isAuthenticated', 'true');
    localStorage.setItem('user', JSON.stringify(userData));
};

// Function to log someone out
export const logout = () => {
    isAuthenticated.set(false);
    user.set(null);
    localStorage.removeItem('isAuthenticated');
    localStorage.removeItem('user');
};

// Function to check if someone was already logged in
export const checkAuth = () => {
    const stored = localStorage.getItem('isAuthenticated');
    const storedUser = localStorage.getItem('user');

    if (stored === 'true' && storedUser) {
        isAuthenticated.set(true);
        user.set(JSON.parse(storedUser));
    }
};
