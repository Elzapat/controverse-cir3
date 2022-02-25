let modal = document.getElementById("modal");
let image_texts = document.getElementsByClassName("image-text");
let shown_element = null;

for (let image of image_texts) {
    image.onclick = () => {
        if (shown_element === null) {
            show_paragraph(image);
        } else {
            hide_paragraph(shown_element);
        }
    };
}

document.onkeydown = e => {
    if (e.key === "Escape" && shown_element !== null) {
        hide_paragraph(shown_element);
    }
}

document.getElementById("read-close").onclick = () => {
    if (shown_element !== null) {
        hide_paragraph(shown_element);
    }
};

function hide_paragraph(elem) {
    shown_element = null;

    document.getElementById("read-close").style.transform = "scale(0)";
    document.querySelectorAll(`#${elem.id} p`).forEach(p => p.style.transform = "scale(0)");
    document.querySelector(".citation").style.transform = "scale(0)";

    elem.classList.remove("shown");
    elem.style.transform = "scale(1) translate(0, 0)";
    setTimeout(() => elem.style.zIndex = "5", 400);
}

function show_paragraph(clicked_elem) {
    shown_element = clicked_elem;

    document.getElementById("read-close").style.transform = "scale(1)";
    document.querySelectorAll(`#${clicked_elem.id} p`).forEach(p => p.style.transform = "scale(1)");
    document.querySelector(".citation").style.transform = "scale(1)";

    clicked_elem.classList.add("shown");
    clicked_elem.style.zIndex = "7";
    switch (clicked_elem.id) {
        case "introduction":
            clicked_elem.style.transform = `scale(2) translate(25%, 25%)`;
            break;
        case "conditions":
            clicked_elem.style.transform = `scale(2) translate(-25%, 25%)`;
            break;
        case "enjeux":
            clicked_elem.style.transform = `scale(2) translate(25%, -25%)`;
            break;
        case "inquietudes":
            clicked_elem.style.transform = `scale(2) translate(-25%, -25%)`;
            break;
    }
}
