var currentPage;

if (window.localStorage.getItem("foods") == null) {
    window.localStorage.setItem("foods", JSON.stringify([]));
}

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
    switch (page) {
        case 0:
            fetch("api/pages/modal/calories").then(response => {
                let data = response.text();
                data.then(contents => {
                    body.innerHTML = contents;
                    let foods = JSON.parse(window.localStorage.getItem("foods"));
                    foods.forEach(food => {
                        addFoodBack(food.name, food.cals);
                    });
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
    let parts = element.parentElement.childNodes[1].data.split(": ");
    element.parentElement.remove();
    let foods = JSON.parse(window.localStorage.getItem("foods"));
    for (var i = 0; i < foods.length; i++) {
        if (foods[i].name == parts[0] && foods[i].cals == parts[1]) {
            foods.splice(i, 1);
        }
    }
    window.localStorage.setItem("foods", JSON.stringify(foods));
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

function addFood() {
    var name = <HTMLInputElement> document.getElementById("FoodName");
    var cals = <HTMLInputElement> document.getElementById("FoodCals");
    let button = <HTMLButtonElement> document.createElement('button');
    button.textContent = "-";
    button.setAttribute("onclick", "removeFood(this)");
    let li = document.createElement("li");
    li.appendChild(button);
    let text = document.createTextNode(`${name.value}: ${cals.value}`);
    li.appendChild(text)
    document.getElementById("itemList").appendChild(li);
    let foods = JSON.parse(window.localStorage.getItem("foods"));
    foods.push({"name": name.value, "cals": cals.value});
    window.localStorage.setItem("foods", JSON.stringify(foods));
    (<HTMLInputElement> document.getElementById("FoodName")).value = "";
    (<HTMLInputElement> document.getElementById("FoodCals")).value = "";
}

function addFoodBack(name: String, cals: Number) {
    let button = <HTMLButtonElement> document.createElement('button');
    button.textContent = "-";
    button.setAttribute("onclick", "removeFood(this)");
    let li = document.createElement("li");
    li.appendChild(button);
    let text = document.createTextNode(`${name}: ${cals}`);
    li.appendChild(text)
    document.getElementById("itemList").appendChild(li);
}

function update() {
    
}