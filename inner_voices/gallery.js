
var btn_next = document.querySelector(".next")
var btn_prev = document.querySelector(".previous")
var gallery_container = document.querySelector(".gallery-container")
var gallery_counter = document.querySelector(".gallery-counter")

var gallery_elements = []
var gallery_counter_elements = []

gallery_index = 0
gallery_max_index = 5

var gallery_img_list = [
	"../gallery/img/invo_new_horizon_heroes.png",
	"../gallery/img/invo_ollie_room.png",
	"../gallery/img/invo_vs_magnum_opus_1.png",
	"screenshots/diner2.png",
	"../gallery/img/invo_mystery_chat.png",
	"../gallery/img/invo_hank_minecart.png",
	"../gallery/img/invo_twisted_school_0.png",
	"../gallery/img/invo_vs_lady_sucrose.png",
	"../gallery/img/invo_ollie_sunlight.png",
]

var populate_gallery = () => {
	gallery_img_list.forEach((el, index) => {
		var img = document.createElement("img")
		img.src = el
		gallery_container.appendChild(img)
		gallery_elements.push(img)

		//<div class="gallery-counter-item"></div>
		var img_counter = document.createElement("div")
		img_counter.classList.add("gallery-counter-item")
		img_counter.onclick = () => {set_gallery_item(index)}
		gallery_counter.appendChild(img_counter)
		gallery_counter_elements.push(img_counter)
	})
	gallery_max_index = gallery_img_list.length - 1
}

var next_gallery_item = () => {
	if (gallery_index < gallery_max_index) gallery_index++
	gallery_adjust_scroll()
}

var prev_gallery_item = () => {
	if (gallery_index > 0) gallery_index--
	gallery_adjust_scroll()
}

var set_gallery_item = (index) => {
	gallery_index = index
	gallery_adjust_scroll()
}

var gallery_adjust_scroll = () => {
	gallery_elements[gallery_index].scrollIntoView({block: "nearest", inline: "nearest"})
	gallery_counter_elements.forEach((el,index)=> {
		el.classList.remove('current')
		if (index == gallery_index)
			el.classList.add('current')
	})
}

addEventListener("load", setup)
addEventListener("load", populate_gallery)
addEventListener("load", gallery_adjust_scroll)

var setup = () => {
	btn_prev.onclick = prev_gallery_item
	btn_next.onclick = next_gallery_item
	gallery_adjust_scroll()
}