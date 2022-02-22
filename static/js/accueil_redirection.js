for (let elem of document.getElementsByClassName("image-text")) {
    elem.onclick = () => {
        window.location.href = elem.id;
    };
}
