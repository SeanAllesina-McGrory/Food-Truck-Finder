<script setup>
defineProps({
	vendor: String,
})
</script>
<template>
	<div class="popup">
		<h1>{{ vendor }}</h1>

		<p>Info about the vendor</p>
	</div>
</template>
<style scoped>
.popup {
	top: 0;
	position: fixed;
	background: rgba(0, 0, 0, 0.5);
	width: 100%;
	height: 100%;
	display: flex;
	justify-content: center;
	align-items: center;
}
</style>
