void function fetch_data() {
    let storage = window.localStorage;
    let data = fetch('/api/current');
    data.then(response => {
        response.json().then(contents => {
            console.log(contents);
        });
    });
}

void function swap_page(page: Number) {
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
            break;
        case 1:
            title.innerHTML = "Calorie Tracking";
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