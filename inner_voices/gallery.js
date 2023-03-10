
var btn_next = document.querySelector(".next")
var btn_prev = document.querySelector(".previous")
var gallery_container = document.querySelector(".gallery-container")

gallery_index = 0
gallery_max_index = 5

var gallery_img_list = [
	"../gallery/img/invo_new_horizon_heroes.png",
	"../gallery/img/invo_ollie_room.png",
	"../gallery/img/invo_vs_magnum_opus_1.png",
	"../gallery/img/invo_mystery_chat.png",
	"../gallery/img/invo_hank_minecart.png",
	"../gallery/img/invo_twisted_school_0.png",
	"../gallery/img/invo_vs_lady_sucrose.png",
	"../gallery/img/invo_ollie_sunlight.png",
]

var populate_gallery = () => {
	gallery_img_list.forEach((el, index) => {
		var img = document.createElement("img")
		gallery_container.appendChild(img)

		img.src = el
	})
	gallery_max_index = gallery_img_list.length
}

var next_gallery_item = () => {
	if (gallery_index < gallery_max_index)
		gallery_index++
	
	gallery_adjust_scroll()
}

var prev_gallery_item = () => {
	if (gallery_index > 0)
		gallery_index--
	
	gallery_adjust_scroll()
}

var gallery_adjust_scroll = () => {
	gallery_container.scroll(gallery_container.clientWidth * gallery_index, 0)
}

addEventListener("load", setup)
addEventListener("load", populate_gallery)
addEventListener("load", gallery_adjust_scroll)

var setup = () => {
	btn_prev.onclick = prev_gallery_item
	btn_next.onclick = next_gallery_item

	gallery_adjust_scroll()
}