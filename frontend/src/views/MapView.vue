<script setup lang="ts">
import { ref, onMounted, onUpdated } from 'vue'
import { VMap, VMapOsmTileLayer, VMapZoomControl, VMapPane, VMapPinMarker } from 'vue-map-ui'
import { fetch_events, get_cords } from '../utils/fetch_data.ts'

const center = ref([47.672786, -122.420654])
const zoom = ref(9)
const bounds = ref([[46.475699386607516, -125.92529296875006], [48.84302835299519, -118.91601562500006]])

const events = ref()
const cords = ref(new Map())

// Floors the math cords into a certain bound so the user doesn't scroll infinently
let floor_map_cord = function (cord) {
	return Math.min(Math.max(cord, -180), 180)
}

// Runs whenever the map view is changed
let view_changed = function (event) {
	// TODO: Add zoom limiting functionality, additionally, make the scroll bounding less choppy

	// If they were to go past the edge of the map their longitude would be > 180
	//    So check if its above and loop around minus 1 just to stop the view_changed function from
	//        recusing endlessly
	//    We also change the bounds, not completely necessary but better for overall appearance
	if (Math.abs(event.center.lng) > 180) {
		event.center.lng = floor_map_cord(event.center.lng) * -1
		event.bounds._northEast.lng = floor_map_cord(event.bounds._northEast.lng) * -1
		event.bounds._southWest.lng = floor_map_cord(event.bounds._southWest.lng) * -1
	}
	// Updates the center and bounds of the map
	center.value = event.center;
	bounds.value = event.bounds;
}

// Logic to run when page is mounted
onMounted(() => {
	// Requests the events from the API then operates on the JSON data returned
	fetch_events().then((response) => {
		// Save the response as the events data
		events.value = response
		// Iterate over the events
		events.value.forEach((event) => {
			// Save the event cords into an array
			const event_cords = [event["cord_x"], event["cord_y"]]
			// Save the array into a hashmap for later use
			cords.value.set(event.uuid, event_cords)
		})
	})
})



</script>

<template>
	<div>
		<p>{{ cords.values }}</p>
		<p v-for="event in events">{{ cords.get(event.uuid) }}</p>
	</div>
	<div id="main">
		<div id="map">
			<VMap id="vmap" :zoom='zoom' :center='center' theme='dark' @view-changed="event => view_changed(event)"
				:attributionControl="true">
				<VMapOsmTileLayer />
				<VMapZoomControl />
				<VMapPane name="events_pane" :z-index="601" />
				<!--I am still unsure what the z-index does, its something about rendering but who knows honestly-->
				<VMapPinMarker pane="events_pane" v-for="event in events" :key="event.uuid"
					:latlng="cords.get(event.uuid)">
				</VMapPinMarker>
			</VMap>
		</div>
	</div>
</template>
<style>
#map {
	height: 90vh;
	width: 100vw;
	background-color: black;
}
</style>
