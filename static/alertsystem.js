let box = document.getElementById("alert-box")

if (box.innerHTML.trim() !== "") {
    box.style.opacity = "100%";
}

async function hideAlert() {
    await new Promise(resolve => setTimeout(resolve, 3000));
    box.style.opacity = "0%";
}

hideAlert()