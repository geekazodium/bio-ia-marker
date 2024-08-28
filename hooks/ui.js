function check_component_exists(s){
    let element = document.getElementById(s);
    return element != null;
}

function extern_create_child(parent_id, html, id){
    let element = document.getElementById(parent_id);
    if(element == null){
        return false;
    }
    let c = document.createElement("div");
    c.innerHTML = html;
    c.id = id;
    element.appendChild(c);
    return true;
}

function extern_move_after(self_id, other_id){
    let selfElement = document.getElementById(self_id);
    if(selfElement == null){
        return false;
    }
    let otherElement = document.getElementById(other_id);
    if(otherElement == null){
        return false;
    }
    otherElement.after(selfElement);
    return true;
}

function extern_restyle(id, style){
    let element = document.getElementById(id);
    if(element == null){
        return false;
    }
    element.style = style;
    return true;
}