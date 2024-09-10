let state;

function save_val_byte(key,val){
    localStorage.setItem(key,val+"b");
    console.log(localStorage);
}

function read_val_byte(key){
    let v = localStorage.getItem(key);
    if(null == v){
        return 0;
    }
    if(!v.endsWith("b")){
        throw new Error("incorrect formatting for value");
    }
    return parseInt(v.split("b",2)[0]);
}

function save_val_string(key,val){
    localStorage.setItem(key,val+("s"));
}

function read_val_string(key){
    let v = localStorage.getItem(key);
    if(null == v){
        return "";
    }
    if(!v.endsWith("s")){
        throw new Error("incorrect formatting for value");
    }
    return v.slice(0,v.length-1);
}

function save_pdf(bytes){
    //WHY THE HECK DOES JAVASCRIPT MAKE ME PUT A BUFFER IN AN ARRAY INSTEAD OF PASSING IT DIRECTLY?????!?!?
    //fix: https://stackoverflow.com/questions/38147480/creating-pdf-blob-from-array-and-viewing-it
    //apparently, new Blob(bytes, {type:"application/pdf"}); breaks for some reason which I STILL can't comprehend
    let blob = new Blob([bytes],{type:"application/pdf"});
    let url = URL.createObjectURL(blob);
    window.open(url);
}