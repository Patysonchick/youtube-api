const { invoke } = window.__TAURI__.tauri

function getViews() {
    let URL = document.getElementById('URLField').value;

    invoke('get_views', { rawVideoId: URL })
        .then((response) => {
            console.log(response)
            alert(response)
            window.viewValue.innerHTML = response
        })
}
