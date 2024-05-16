const { appWindow } = window.__TAURI__.window
const { invoke } = window.__TAURI__.tauri
const { message } = window.__TAURI__.dialog

document
    .getElementById('titlebar-minimize')
    .addEventListener('click', () => appWindow.minimize())
document
    .getElementById('titlebar-maximize')
    .addEventListener('click', () => appWindow.toggleMaximize())
document
    .getElementById('titlebar-close')
    .addEventListener('click', () => appWindow.close())

invoke('get_pkg_version')
    .then(async (response) => {
        window.versionid.innerHTML = response
    })

invoke('get_token')
    .catch(async (error) => {
        message('No token', { title: 'Error', type: 'error' });
    })

function getViews() {
    window.resulttitle.innerHTML = "Views"

    let URL = document.getElementById('urlfield').value
    invoke('get_views', { rawVideoId: URL })
        .then(async (response) => {
            console.log(response)
            window.result.innerHTML = response
            message(response, 'Views')
        })
        .catch(async (error) => {
            message(error, { title: 'Error', type: 'error' });
        })
}
