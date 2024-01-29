export async function fetch_vendors() {
	const vendors = await fetch("http://localhost:8080/api/vendors").then((response) => {
		if (!response.ok) {
			throw new Error(`Error ${response.status}`)
		}
		return response.json()
	})
	.catch(() => {
		console.error('Failed to retrieve vendor data')
	})

	return vendors
}

export async function fetch_events() {
	const events = await fetch("http://localhost:8080/api/events").then((response) => {
		if (!response.ok) {
			throw new Error(`Error ${response.status}`)
		}
		return response.json()
	})
	.catch(() => {
		console.error('Failed to retrieve event data')
	})

	return events 
}

export async function get_cords(address) {
	console.log(`Address: ${address}`)
	const cords = await fetch(`http://localhost:5173/geocoder/locations/onelineaddress?address=${address}&benchmark=4&format=json`)
		.then((response) => {
		if (!response.ok) {
			console.log(response)
			throw new Error(`Error ${response.status}`)
		}
		return response.json()
	})
	.catch(() => {
		console.error('Failed to convert address into geocords')
	})
	console.log(cords)
	return cords
}
