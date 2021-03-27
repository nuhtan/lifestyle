var currentPage;

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
    currentPage = page;
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
            fetch("api/pages/calories").then(response => {
                let data = response.text();
                data.then(contents => {
                    body.innerHTML = contents;
                });
            });
            break;
        case 2:
            title.innerHTML = "Long Term Shopping";
            break;
        case 3:
            title.innerHTML = "Valorant Rank Progress";
            break;
        case 4:
            title.innerHTML = "Website Progress";
            fetch("api/pages/progress").then(response => {
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

function choose_modal(page: Number) {
    let body = document.getElementById("modal-body");
    console.log(page);
    switch (page) {
        case 0:
            fetch("api/pages/modal/calories").then(response => {
                let data = response.text();
                data.then(contents => {
                    body.innerHTML = contents;
                });
            });
            break;
        case 1:
            fetch("api/pages/modal/progress").then(response => {
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

var modal;
var btn;
var closeButton;

function modalSetup(modalPage: Number) {
    modal = document.getElementById("myModal");
    btn = document.getElementById("add");
    closeButton = document.getElementsByClassName("close")[0];

    btn.onclick = function () {
        modal.style.display = "block";
        choose_modal(modalPage);
    }

    closeButton.onclick = function () {
        modal.style.display = "none";
    }

    window.onclick = function (event) {
        if (event.target == modal) {
            modal.style.display = "none";
        }
    }
}

function removeFood(element) {
    element.parentElement.remove();
}

function toggleToDo(element) {
    console.log(element);
}

function toggleSwitch(element) {
    if (element.attributes["checked"].value == "false") {
        element.attributes["checked"].value = "true";
    } else {
        element.attributes["checked"].value = "false";
    }
}