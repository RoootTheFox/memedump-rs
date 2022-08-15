/*
 used for storing memes so we can play them as well as saving them without having to download them again
 */
let memes = [];

const getJSON = async url => {
    try {
        const response = await fetch(url);
        return await response.json();
    } catch (error) {
        return error;
    }
}

async function loadMeme(id) {
    console.log("loading meme with id " + id);
    return await getJSON("/api/getmeme?id="+id);
}

function playMeme(id) {
    const meme = document.getElementById("meme-"+id);
    console.log(id);
    console.log(meme);

    if (memes.length === 0 || memes["meme-"+id] === undefined) {
        loadMeme(id).then(res => {
            memes["meme-"+id] = res;
            meme.src = b64toBlob(res.data);
            meme.play();
        });
    }
}

// https://stackoverflow.com/a/16245768/19242257
const b64toBlob = (b64Data, contentType='', sliceSize=512) => {
    const byteCharacters = atob(b64Data);
    const byteArrays = [];

    for (let offset = 0; offset < byteCharacters.length; offset += sliceSize) {
        const slice = byteCharacters.slice(offset, offset + sliceSize);

        const byteNumbers = new Array(slice.length);
        for (let i = 0; i < slice.length; i++) {
            byteNumbers[i] = slice.charCodeAt(i);
        }

        const byteArray = new Uint8Array(byteNumbers);
        byteArrays.push(byteArray);
    }

    return new Blob(byteArrays, {type: contentType});
}


getJSON("api/getmemes").then(data => {
    let memeContainer = document.getElementById("memes");
    for(let i = 0; i < data.length; i++) {
        let id = data[i]["id"];
        let title = data[i]["title"];
        let details = data[i]["details"];

        let thumbnail_b64 = "data:image/png;base64," + data[i]["thumbnail"];

        let meme = document.createElement("div");
        meme.className = "meme";
        meme.id = "meme-"+id;

        let meme_thumbnail = document.createElement("img");
        meme_thumbnail.src = thumbnail_b64;
        meme_thumbnail.className = "meme-thumbnail";
        meme_thumbnail.onclick = function() {
            playMeme(id);
        };

        let meme_title = document.createElement("span");
        meme_title.className = "meme-title";
        meme_title.innerHTML = title;

        let meme_details = document.createElement("span");
        meme_details.className = "meme-details";
        meme_details.innerHTML = details;

        meme.appendChild(meme_thumbnail);
        meme.appendChild(meme_title);
        meme.appendChild(meme_details);
        memeContainer.appendChild(meme);
    }
});