function fetch_data() {
    // let storage = window.localStorage;
    // let data = fetch('/api/current');
    // data.then(response => {
    //     response.text().then(content => {
    //         console.log(content);
    //     })
    // });
    swap_page(0);
}
function swap_page(page: Number) {
    let title = document.getElementById("pageTitle");
    let body = document.getElementById("pageBody");
    switch (page) {
        case 0:
            title.innerHTML = "Goal Status";
            fetch("api/pages/goals").then(response => {
                let data = response.text();
                data.then(contents => {
                    body.innerHTML = contents;
                });
            });
            page = 0;
            break;
        case 1:
            title.innerHTML = "Calorie Tracking";
            fetch("api/pages/calories").then(response => {
                let data = response.text();
                data.then(contents => {
                    body.innerHTML = contents;
                });
            });
            page = 1;
            break;
        case 2:
            title.innerHTML = "Long Term Shopping";
            break;
        case 3:
            title.innerHTML = "Valorant Rank Progress";
            break;
        case 4:
            title.innerHTML = "Website Progress";
            break;
        default:
            break;
    }
}

function choose_modal(page: Number) {
    let body = document.getElementById("modal-body");
    switch (page) {
        case 0:
            fetch("api/pages/modal/calories").then(response => {
                let data = response.text();
                data.then(contents => {
                    body.innerHTML = contents;
                });
            });
            break;
        default:
            break;
    }
}