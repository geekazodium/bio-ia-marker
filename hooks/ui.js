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

function extern_listen_for_mouse(id, listener){
    document.getElementById(id).addEventListener("mousedown",(e)=>{listener.on_mouse_down(id);});
    document.getElementById(id).addEventListener("mouseup",(e)=>{listener.on_mouse_up(id);});
}

function extern_set_content_text(id, text){
    document.getElementById(id).innerText = text;
}

function sync_selected_grade_display(){
    document.getElementById('selected_value_display').innerText = document.getElementById('selected_value_slidebar').value;
}

function extern_listen_for_input(id, listener){
    document.getElementById(id).addEventListener("input", (e)=>{listener.on_input(id,document.getElementById(id).value);});
    document.getElementById(id).addEventListener("change", (e)=>{listener.on_change(id,document.getElementById(id).value);});
}

function extern_get_input_value(id){
    return document.getElementById(id).value;
}

function extern_simulate_click(id){//https://stackoverflow.com/questions/2705583/how-to-simulate-a-click-with-javascript
    document.getElementById(id).dispatchEvent(new MouseEvent("mousedown"));
    document.getElementById(id).dispatchEvent(new MouseEvent("mouseup"))
}

function extern_set_input_value(id,value){
    document.getElementById(id).value = value;
    sync_selected_grade_display();
}

function extern_listen_for_window_events(listener){
    //https://stackoverflow.com/questions/13443503/run-javascript-code-on-window-close-or-page-refresh
    document.addEventListener("visibilitychange",()=>{listener.on_window_change()});
    window.addEventListener("beforeunload",()=>{listener.on_window_close()});
}

document.addEventListener("DOMContentLoaded",sync_selected_grade_display);