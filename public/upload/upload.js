function getBase64(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.readAsDataURL(file);
        reader.onload = () => resolve(reader.result);
        reader.onerror = error => reject(error);
    });
}

function load() {
    document.getElementById("upload-btn").onclick = async function () {
        let title = document.getElementById("form-title").value;
        let details = document.getElementById("form-details").value;
        let tags = document.getElementById("form-tags").value.split(",");
        let file = document.getElementById("form-file").files[0]; // data
        let file_type = file.type;

        console.log(file_type);

        let data = await getBase64(file);
        data = data.substring(data.indexOf(",") + 1);

        let request_body = {
            title: title,
            details: details,
            tags: tags,
            data: data,
            datatype: file_type.substring(file_type.indexOf("/") + 1)
        }

        let xhr = new XMLHttpRequest();
        xhr.open("POST", "/api/addmeme");
        xhr.setRequestHeader("Accept", "application/json");
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.send(JSON.stringify(request_body));

        console.log(file);
        console.log("uploading...");
    }
}