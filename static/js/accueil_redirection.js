for (let elem of document.getElementsByClassName("image-text")) {
    if (elem.id === "introduction") {
        continue;
    }

    elem.onclick = () => {
        window.location.href = elem.id;
    };
}
