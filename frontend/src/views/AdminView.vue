<script setup>
import { ref } from 'vue'
import Popup from "../components/Popup.vue"
const visible = ref(false)
const vendor = ref(undefined)
function onClick() {
	vendor.value = document.getElementById("vendor").value
	visible.value = !visible.value
}
</script>
<template>
	<div v-if="visible" @click="visible.value = false">
		<Popup :vendor="vendor" />
	</div>
	<div>
		<form id="submit">
			<div>
				<label for="vendor">Enter name of vendor:</label>
				<input type="text" id="vendor" name="vendor" value="War Pig Smoke House" />
			</div>
			<input type="button" @click.left="onClick" value="Submit" />
		</form>
	</div>
</template>
