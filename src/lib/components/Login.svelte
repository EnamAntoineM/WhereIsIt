<script>
  import { invoke } from "@tauri-apps/api/core";
  import { login } from '../stores/auth.js';

  // Variables to store what user types
  let email = '';
  let password = '';
  let error = '';

  // Function that runs when form is submitted
  const handleLogin = async (e) => {
    e.preventDefault(); // Stop page from refreshing
    error = ''; // Clear any previous errors

    // Check if both fields are filled
    if (!email || !password) {
      error = 'Please enter both email and password';
      return;
    }

    try {
      // For now, we'll use simple check - you can change this later
      if (email === 'admin@example.com' && password === 'admin') {
        // Login successful - tell the brain someone logged in
        login({ email, id: 1 });
      } else {
        error = 'Invalid email or password';
      }
    } catch (err) {
      error = 'Login failed. Please try again.';
    }
  };
</script>

<main class="login-form">
    {#if error}
        <div class="error-message">
            <i class="fas fa-exclamation-triangle"></i>
            {error}
        </div>
    {/if}
	<section class="login-form-container">
		<div class="heading">
			<h1> Join Us </h1>
			<a href="https:github.com/EnamAntoineM" target="_blank">
				<img src="/WIIT-GoldSymbol.png" class="logo" alt="wiit Logo" />
			</a>
		</div>
		<form class="login-form-fields" on:submit={handleLogin}>
                <div class="form-group">
                    <label class="text-md" for="email">
                        <i class="fas fa-user"></i>
                        Email
                    </label>
                    <p class="label-body">
                        <input type="email" id="email" bind:value={email} required placeholder="Enter your email">
                    </p>
                </div>
                <div class="form-group">
                    <label class="text-md" for="password">
                        <i class="fas fa-key"></i>
                        Password
                    </label>
                    <p class="label-body">
                        <input type="password" id="password" bind:value={password} required placeholder="Enter your password">
                    </p>
                </div>
                <button type="submit" class="submit-btn btn btn-primary">
                    Log In
                </button>
                <p class=" sign-up text-sm">
                    Don't have an account? 
                    <a href="#">
                        Sign up
                    </a>
                </p>
        </form>
	</section>
</main>

<style>
/* All your existing styles stay the same, plus this new error style */

.error-message {
    position: absolute;
    top: 0;
    right: 0;
    background-color: #f8f4e9;
    color: #fa0606;
    padding: 10px 20px;
    margin: 15px 13px;
    border-radius: 8px;
    border: 1px solid #f5c6cb;
    display: flex;
    align-items: center;
    gap: 10px;
    max-width: 90%;
}

:global(body) {
    min-height: 90vh;
    min-width: 90vw;
}

:root {
	font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
	font-size: 16px;
	line-height: 24px;
	font-weight: 400;
	--primary-color: #4891ff;
	--light-color: #f4f4f6;
	--dark-color: #111;
	font-synthesis: none;
	text-rendering: optimizeLegibility;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	-webkit-text-size-adjust: 100%;
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: 'Poppins', sans-serif;
    font-size: 16px;
    line-height: 1.5;
    background: #fff;
}

a {
    text-decoration: none;
    color: #333;
}

ul {
    list-style: none;
}

img {
    max-width: 100%;
}

/* UTILITY */

/* =========================
   Buttons
   ========================= */
.btn {
    display: inline-block;
    padding: 13px 20px;
    background: var(--light-color);
    color: #333;
    font-weight: 600;
    text-decoration: none;
    border: none;
    border-radius: 16px;
    cursor: pointer;
    transition: 0.3s;
}

.btn:hover {
    opacity: 0.7;
}

.btn-primary {
    background: var(--primary-color);
    color: rgb(221, 221, 221);
}

.btn-dark {
    background: var(--dark-color);
    color: #fff;
}

.btn-block {
    display: block;
    width: 100%;
}


/* =========================
   Backgrounds
   ========================= */
.bg-primary {
    background: var(--primary-color);
    color: #fff;
}

.bg-light {
    background: var(--light-color);
    color: #333;
}

.bg-dark {
    background: var(--dark-color);
    color: #fff;
}

.bg-black {
    background: black;
    color: #fff;
}

/* =========================
   Utility Classes
   ========================= */
.container {
    max-width: 1100px;
    margin: 0 auto;
    padding: 0 15px;
}

.container-sm {
    max-width: 800px;
    margin: 0 auto;
    padding: 0 15px;
}

.text-xxl {
    font-size: 3rem;
    line-height: 1.2;
    font-weight: 600;
    margin: 40px 0 20px;
}

.text-xl {
    font-size: 2.2rem;
    line-height: 1.4;
    font-weight: normal;
    margin: 40px 0 20px;
}

.text-lg {
    font-size: 1.8rem;
    line-height: 1.4;
    font-weight: normal;
    margin: 30px 0 20px;
}

.text-md {
    font-size: 1.2rem;
    line-height: 1.4;
    font-weight: normal;
    margin: 20px 0 10px;
}

.text-sm {
    font-size: 1rem;
    line-height: 1.4;
    font-weight: normal;
    margin: 10px 0 5px;
}

.text-center {
    text-align: center;
}


/* LOGIN FORM STYLE */

.login-form .heading .logo {
	padding: 0.8em;
	height: 68px;
	width: 78px;
	will-change: filter;
	transition: 0.75s;
}

.login-form .heading .logo .logo:hover {
  	filter: drop-shadow(0 0 5em #c08d52);
}

.login-form {
    position: fixed;
    top: 0;
    left: 0;
    min-width: 100vw;
    min-height: 100vh;
    width: 100vw;
    height: 100vh;
    background: url("/bg-login.jpg") no-repeat center center fixed;
    background-size: cover;
    object-fit: cover;
    display: flex;
    align-items: center;
    justify-content: center;
}

.login-form .login-form-container {
    background-color: #fff;
    opacity: 0.96;
	padding: 10px 10px;
    border: 1px solid var(--dark-color);
    width: 500px;
    height: 560px;
    border-radius: 20px;
	flex-direction: column;
    align-items: center;
}

.login-form .login-form-container .heading {
	padding-top: 20px;
	display: flex;
	justify-content: center;
	gap: 0;
  	align-items: center; /* Vertically center items */
}

.login-form .login-form-container .heading h1 {
	margin: 0 0 0 0; /* Remove default margin and add right spacing */
	font-size: 2.2rem; /* Optional: adjust heading size */
}

.login-form .login-form-container .heading a {
	margin-left: 0px;
}

.login-form .login-form-container .form-group {
	display: flex;
	flex-direction: column;
	align-items: left;
    padding: 5px 20px;
    width: 100%;
}

.login-form .login-form-container .form-group label {
    display: flex;
	align-items: center;
    text-align: left;
    margin-left: 15px;
    font-weight: 500;
}

.login-form .login-form-container .form-group label i{
    margin-left: 8px;
    margin-right: 8px;
}

.login-form .login-form-container .form-group .label-body input {
    width: 100%;
    padding: 15px;
	margin: 15px 3px;
    border-radius: 16px;
}

.login-form .login-form-container .login-form-fields .submit-btn {
	margin: 11px;
	width: 90%;
}

.login-form .login-form-container .sign-up{
    cursor: pointer;
    padding: 9px 90px;
}

.login-form .login-form-container .sign-up a:hover{
    color: var(--primary-color);
    text-decoration: underline;
    transition: color 0.2s;
}

.login-form .login-form-container .login-form-heading {
    color: var(--dark-color);
    margin-left: auto;
    margin-right: auto;
}

.login-form .login-form-container .login-form-fields {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}
</style>
