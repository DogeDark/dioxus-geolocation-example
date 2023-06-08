let map;

export function initMap() {
  initMapAsync();
}

async function initMapAsync() {
  const { Map } = await google.maps.importLibrary("maps");
  map = new Map(document.getElementById("map-div"), {
    zoom: 16,
  });
}

export function panTo(lat, lng) {
  if (map) {
    map.panTo({ lat: lat, lng: lng });
  }
}