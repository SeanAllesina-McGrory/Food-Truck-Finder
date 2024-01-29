<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
	<header>
		<div class="wrapper">
			<nav class="navbar">
				<RouterLink to="/">Map</RouterLink>
				<RouterLink to="/test">Test</RouterLink>
				<RouterLink to="/balderdash">Admin</RouterLink>
				<RouterLink to="/login">Login</RouterLink>
				<RouterLink to="/register">Register</RouterLink>
				<RouterLink to="/vendors">Vendor Info</RouterLink>
				<RouterLink to="/manage">Account Management</RouterLink>
			</nav>
		</div>
	</header>

	<RouterView />
</template>
<style scoped>
.navbar {
	font-size: 24pt;
	padding: 10px;
	margin-right: 10px;
	display: flex;
	align-items: space-evenly;
	justify-content: space-evenly;
}
</style>
