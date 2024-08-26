let state;

function save_val_byte(key,val){
    localStorage.setItem(key,val+"b");
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