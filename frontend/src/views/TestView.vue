<script setup>
import { ref, onMounted, onUpdated } from 'vue'
import { fetch_vendors } from '../utils/fetch_data.ts'
const vendors = ref()
const filtered_vendors = ref()

const filter_search = function () {
	let search_term = document.getElementById("filter").value.toLowerCase()
	filtered_vendors.value = vendors.value.filter(vendor => {
		return vendor.name.toLowerCase().includes(search_term)
	})
	console.log(filtered_vendors.value)
}

onMounted(() => {
	fetch_vendors().then((response) => {
		vendors.value = response
		filter_search()
	})
})
</script>
<template>
	<div>
		<h1>Filters</h1>
		<input type="text" id="filter" @keyup="filter_search()" placeholder="Search for vendors">

	</div>
	<div>
		<table>
			<tr>
				<th>UUID</th>
				<th>name</th>
				<th>description</th>
				<th>vendor type</th>
				<th>email</th>
				<th>phone number</th>
				<th>website</th>
			</tr>
			<tr v-for="vendor in filtered_vendors">
				<th>{{ vendor.uuid }}</th>
				<th>{{ vendor.name }}</th>
				<th>{{ vendor.description }}</th>
				<th>{{ vendor.vendor_type }}</th>
				<th>{{ vendor.email }}</th>
				<th>{{ vendor.phone_number }}</th>
				<th>{{ vendor.website }}</th>
			</tr>
		</table>
	</div>
</template>
