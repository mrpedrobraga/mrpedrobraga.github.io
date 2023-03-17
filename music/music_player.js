function AudioPlayer () {
	this.track = {
		"album": "Inner Voices: Remember Me",
		"author": "Pedro Braga",
		"name": "Cursed Carnival",
		"file": "mus/Cursed Carnival.mp3",
		"img": "cover-very-necessary-tension.png",
		"bg": "cover-very-necessary-tension-bg.png"
	}

	this.player = new Audio(this.track["file"])

    this.setup = () => {
        
        // PLAY/PAUSE TOGGLE!!!
        this.btn_play.onclick = () => {
            let music_playing = !this.player.paused

            if (music_playing) this.player.pause()
            else this.player.play()
            
            this.btn_play.classList.remove('playing')
            if (!this.player.paused) this.btn_play.classList.add('playing')
        }

        // UPDATE TIME
        this.player.ontimeupdate = () => {
            let percentage = this.player.currentTime / this.player.duration
            this.track_progress.style.width = (percentage * 100).toString() + "%"
            this.track_progress_counter.textContent = `${this.player.currentTime.toString().toMMSS()} / ${this.player.duration.toString().toMMSS()}`
        }

        // RESET PLAY BTN WHEN PAUSING
        // RESET BAR WHEN MUSIC FINISHES
        this.player.onpause = () => {
            this.btn_play.classList.remove('playing')
            if (this.player.currentTime >= this.player.duration)
                this.track_progress.style.width = "0%"
                this.player.currentTime = 0
        }

        // SEEKING
        this.track_container.onclick = (ev) => {
            let percentage = ev.layerX / this.track_container.offsetWidth
            this.player.currentTime = this.player.duration * percentage
        }
    }
}

var AudioPlayer = new AudioPlayer()

AudioPlayer.btn_play = document.querySelector("#play")
AudioPlayer.btn_mute = document.querySelector("#mute")
AudioPlayer.btn_shuffle = document.querySelector("#shuffle")

AudioPlayer.track_container = document.querySelector("#music-player-track")
AudioPlayer.track_progress = document.querySelector("#music-player-track-progress")
AudioPlayer.track_progress_counter = document.querySelector("#track-time")

AudioPlayer.setup()


String.prototype.toHHMMSS = function () {
    var sec_num = parseInt(this, 10); // don't forget the second param
    var hours   = Math.floor(sec_num / 3600);
    var minutes = Math.floor((sec_num - (hours * 3600)) / 60);
    var seconds = sec_num - (hours * 3600) - (minutes * 60);

    if (hours   < 10) {hours   = "0"+hours;}
    if (minutes < 10) {minutes = "0"+minutes;}
    if (seconds < 10) {seconds = "0"+seconds;}
    return hours+':'+minutes+':'+seconds;
}

String.prototype.toMMSS = function () {
    var sec_num = parseInt(this, 10); // don't forget the second param
    var minutes = Math.floor((sec_num) / 60);
    var seconds = sec_num - (minutes * 60);

    if (minutes < 10) {minutes = "0"+minutes;}
    if (seconds < 10) {seconds = "0"+seconds;}
    return minutes+':'+seconds;
}