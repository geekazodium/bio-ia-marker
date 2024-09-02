
use core::panic;
use std::{cell::RefCell, rc::Rc, usize, vec};

use wasm_bindgen::prelude::*;

static GRADE_EXISTS_FLAG:u8 = 0b10000000;
static GRADE_SELECTED_VALUE_SLIDEBAR_ID:&str = "selected_value_slidebar";

static SUB_STRAND_HTML: &str = "
<li class=\"dropdown_parent_bar\"> 
    <h3>
        placeholder_strand
        <img src = \"images/dropdown_triangle.svg\" width=\"10px\" height=\"10px\">
    </h3>
</li>";

static OVERALL_STRAND_HTML: &str ="
<li class=\"dropdown_parent_bar\">
    <h2>
        placeholder_strand
        <img src = \"images/dropdown_triangle.svg\" width=\"10px\" height=\"10px\">
    </h2>
</li>
";

#[wasm_bindgen]
extern {
    pub fn check_component_exists(s: &str) -> bool;
    pub fn extern_create_child(parent_id: &str, html: &str, id: &str) -> bool;
    pub fn extern_move_after(this: &str, other: &str) -> bool;
    pub fn extern_restyle(id: &str, style: &str) -> bool;
    pub fn extern_listen_for_mouse(id: &str, listener: MouseEventInterface);
    pub fn extern_listen_for_input(id: &str, listener: InputEventInterface);
    pub fn extern_set_content_text(id: &str, text: &str);
    pub fn extern_simulate_click(id: &str);
    pub fn js_reset_anim(id: &str);
    pub fn js_delayed_set_height(id: &str, h:&str, delay: f32);
    pub fn alert(s: &str);
    pub fn log(s: &str);
    pub fn crash(s: &str);
    pub fn save_val_byte(k: &str, v: u8);
    pub fn read_val_byte(k: &str) -> u8;
    pub fn save_val_string(k: &str, v: &str);
    pub fn read_val_string(k: &str) -> String;
}

#[wasm_bindgen]
pub fn create_state()-> State{
    return State{student_grades:None};
}

#[wasm_bindgen]
pub struct State{
    student_grades:Option<Rc<RefCell<StudentGrades>>>
}

#[wasm_bindgen]
impl State {
    pub fn load_last_grades(&mut self){
        let grades = StudentGrades::new();
        grades.borrow_mut().load_last_grades();
        self.student_grades = Some(grades);
    }
    pub fn write_grade(&mut self,index: usize, val: u8){
        save_val_byte(&StrandName::from_index(index).to_string_v(), val | GRADE_EXISTS_FLAG)
    }
}

pub struct StudentGrades{
    overall:Vec<OverallCriteria>,
    current_selected_overall:Option<usize>,
    last_displayed_substrand:Option<Vec<u8>>,
    strand_display_elements:Vec<HtmlComponent>
}

impl StudentGrades {
    fn new()->Rc<RefCell<StudentGrades>>{
        let attempt = HtmlComponent::new_from_document("strand_selection".to_string());
        if attempt.is_none() {crash("document is missing component"); panic!("");}
        let menu_parent = attempt.unwrap();
        let overall_criteria = vec![
            OverallCriteria::new(StrandName::ResearchDesignOverall,vec![
                NumberedStrand::new(StrandName::ResearchDesign1,&menu_parent),
                NumberedStrand::new(StrandName::ResearchDesign2,&menu_parent),
                NumberedStrand::new(StrandName::ResearchDesign3,&menu_parent)
            ],&menu_parent),
            OverallCriteria::new(StrandName::DataAnalysisOverall,vec![
                NumberedStrand::new(StrandName::DataAnalysis1,&menu_parent),
                NumberedStrand::new(StrandName::DataAnalysis2,&menu_parent),
                NumberedStrand::new(StrandName::DataAnalysis3,&menu_parent)
            ],&menu_parent),
            OverallCriteria::new(StrandName::ConclusionOverall,vec![
                NumberedStrand::new(StrandName::Conclusion1,&menu_parent),
                NumberedStrand::new(StrandName::Conclusion2,&menu_parent)
            ],&menu_parent),
            OverallCriteria::new(StrandName::EvaluationOverall,vec![
                NumberedStrand::new(StrandName::Evaluation1,&menu_parent),
                NumberedStrand::new(StrandName::Evaluation2,&menu_parent)
            ],&menu_parent)
        ];
        let ret: Rc<RefCell<StudentGrades>> = Rc::new(RefCell::new(StudentGrades{
            overall:overall_criteria,
            current_selected_overall:Some(0),
            last_displayed_substrand:None,
            strand_display_elements:vec![
                HtmlComponent::new_from_document("strand_low".to_string()).unwrap(),
                HtmlComponent::new_from_document("strand_mid".to_string()).unwrap(),
                HtmlComponent::new_from_document("strand_high".to_string()).unwrap()
            ]
        }));
        let mut c1 = 0;
        for s in &ret.borrow().overall{
            extern_listen_for_mouse(&s.ui_component.html_id,MouseEventInterface::new(ret.clone(),vec![c1]));
            let mut c2 = 0;
            for c in &s.strands{
                extern_listen_for_mouse(&c.ui_component.html_id, MouseEventInterface::new(ret.clone(),vec![c1,c2]));
                c2+=1;
            }
            c1+=1;
        }
        extern_listen_for_input(GRADE_SELECTED_VALUE_SLIDEBAR_ID, InputEventInterface::new(ret.clone()));
        let id = ret.borrow().overall.get(0).map( |v|&v.ui_component.html_id).unwrap().to_owned();
        extern_simulate_click(&id);
        ret
    }
    fn load_last_grades(&mut self){
        for strand in &mut self.overall{
            let s = strand;
            s.load_saved();
            for sub_strand in &mut s.strands{
                sub_strand.load_saved();
            }
        }
    }
    fn select_overall(&mut self,i: usize){
        if i>=self.overall.len(){
            return;
        }
        self.overall[i].show_sub_strands();
        self.current_selected_overall = Some(i);
    }
    fn unselect_overall(&mut self){
        if self.current_selected_overall.is_none(){
            return;
        }
        self.overall[self.current_selected_overall.unwrap()].hide_sub_strands();
        self.current_selected_overall = None;
    }
}

impl MouseEventListener for StudentGrades{
    fn on_mouse_down(&mut self, _id: &str, interface_id: &Vec<u8>) {
        let overall_idx = *interface_id.get(0).unwrap() as usize;
        if interface_id.len() == 1 {
            if self.current_selected_overall.is_some() {
                self.unselect_overall();
            }
            self.select_overall(overall_idx);
        }else if interface_id.len() == 2 { 
            self.last_displayed_substrand = Some(interface_id.to_owned());
            let criteria = self.overall
                .get(overall_idx).unwrap()
                .strands.get(*interface_id.get(1).unwrap() as usize).unwrap()
                .get_strand_name().to_criteria();
            for i in 0..usize::min(criteria.len(),self.strand_display_elements.len()){
                self.strand_display_elements[i].set_content_text(criteria[i]);
            }
        }
    }
    fn on_mouse_up(&mut self, _id: &str, _interface_id: &Vec<u8>) {
        
    }
}

impl InputEventListener for StudentGrades{
    fn on_input(&mut self, _id: &str, _data: &str) {
        log("hello");
    }
    fn on_change(&mut self, id: &str, data: &str){
        if self.last_displayed_substrand.is_none(){
            return;
        }
        if id == GRADE_SELECTED_VALUE_SLIDEBAR_ID {
            let res = u8::from_str_radix(data, 10);
            if res.is_ok(){
                let g = res.unwrap();
                let substrand_id = self.last_displayed_substrand.as_ref().unwrap();
                let overall = self.overall.get_mut(substrand_id.get(0).unwrap_or(&0).to_owned() as usize).unwrap();
                if substrand_id.len() == 2{
                    let sub = overall.strands.get_mut(substrand_id.get(1).unwrap_or(&0).to_owned() as usize).unwrap();
                    sub.assign_value(g);
                    sub.save_value();
                }
            }
        }
    }
}

enum StrandName {
    ResearchDesignOverall,
    ResearchDesign1,
    ResearchDesign2,
    ResearchDesign3,
    DataAnalysisOverall,
    DataAnalysis1,
    DataAnalysis2,
    DataAnalysis3,
    ConclusionOverall,
    Conclusion1,
    Conclusion2,
    EvaluationOverall,
    Evaluation1,
    Evaluation2
}

impl StrandName{
    fn to_string(&self)->&str{
        match *self {
            Self::ResearchDesignOverall => "Research Design Overall",
            Self::ResearchDesign1 => "Research Design 1",
            Self::ResearchDesign2 => "Research Design 2",
            Self::ResearchDesign3 => "Research Design 3",
            Self::DataAnalysisOverall => "Data Analysis Overall",
            Self::DataAnalysis1 => "Data Analysis 1",
            Self::DataAnalysis2 => "Data Analysis 2",
            Self::DataAnalysis3 => "Data Analysis 3",
            Self::ConclusionOverall => "Conclusion Overall",
            Self::Conclusion1 => "Conclusion 1",
            Self::Conclusion2 => "Conclusion 2",
            Self::EvaluationOverall => "Evaluation Overall",
            Self::Evaluation1 => "Evaluation 1",
            Self::Evaluation2 => "Evaluation 2"
        }
    }
    fn to_criteria(&self) -> Vec<&str>{
        match *self {
            Self::ResearchDesignOverall => vec![],
            Self::ResearchDesign1 => vec!["The research question is stated without context.","The research question is outlined within a broad context.","The research question is described within a specific and appropriate context."],
            Self::ResearchDesign2 => vec!["Methodological considerations associated with collecting data relevant to the research question are stated.","Methodological considerations associated with collecting relevant and sufficient data to answer the research question are described.","Methodological considerations associated with collecting relevant and sufficient data to answer the research question are explained."],
            Self::ResearchDesign3 => vec!["The description of the methodology for collecting or selecting data lacks the detail to allow for the investigation to be reproduced.","The description of the methodology for collecting or selecting data allows for the investigation to be reproduced with few ambiguities or omissions.","The description of the methodology for collecting or selecting data allows for the investigation to be reproduced."],
            Self::DataAnalysisOverall => vec![],
            Self::DataAnalysis1 => vec!["The recording and processing of the data is communicated but is neither clear nor precise.","The communication of the recording and processing of the data is either clear or precise.","The communication of the recording and processing of the data is both clear and precise."],
            Self::DataAnalysis2 => vec!["The recording and processing of data shows limited evidence of the consideration of uncertainties.","The recording and processing of data shows evidence of a consideration of uncertainties but with some significant omissions or inaccuracies.","The recording and processing of data shows evidence of an appropriate consideration of uncertainties."],
            Self::DataAnalysis3 => vec!["Some processing of data relevant to addressing the research question is carried out but with major omissions, inaccuracies or inconsistencies","The processing of data relevant to addressing the research question is carried out but with some significant omissions, inaccuracies or inconsistencies.","The processing of data relevant to addressing the research question is carried out appropriately and accurately."],
            Self::ConclusionOverall => vec![],
            Self::Conclusion1 => vec!["A conclusion is stated that is relevant to the research question but is not supported by the analysis presented.","A conclusion is described that is relevant to the research question but is not fully consistent with the analysis presented.","A conclusion is justified that is relevant to the research question and fully consistent with the analysis presented."],
            Self::Conclusion2 => vec!["The conclusion makes superficial comparison to the accepted scientific context.","A conclusion is described that makes some relevant comparison to the accepted scientific context.","A conclusion is justified through relevant comparison to the accepted scientific context."],
            Self::EvaluationOverall => vec![],
            Self::Evaluation1 => vec!["The report states generic methodological weaknesses or limitations.","The report describes specific methodological weaknesses or limitations.","The report explains the relative impact of specific methodological weaknesses or limitations."],
            Self::Evaluation2 => vec!["Realistic improvements to the investigation are stated.","Realistic improvements to the investigation that are relevant to the identified weaknesses or limitations, are described.","Realistic improvements to the investigation, that are relevant to the identified weaknesses or limitations, are explained."]
        }
    }
    fn to_string_v(&self)->String{
        self.to_string().to_owned() + "v"
    }
    fn from_index(index: usize)->StrandName{ //hackey whachkey method
        match index{
            1 => Self::ResearchDesignOverall,
            2 => Self::ResearchDesign1,
            3 => Self::ResearchDesign2,
            4 => Self::ResearchDesign3,
            5 => Self::DataAnalysisOverall,
            6 => Self::DataAnalysis1,
            7 => Self::DataAnalysis2,
            8 => Self::DataAnalysis3,
            9 => Self::ConclusionOverall,
            10 => Self::Conclusion1,
            11 => Self::Conclusion2,
            12 => Self::EvaluationOverall,
            13 => Self::Evaluation1,
            14 => Self::Evaluation2,
            _default => {crash("invalid index");panic!("invalid index")}
        }
    }
}

trait Grade {
    fn assign_value(&mut self,val: u8);
    fn clear_value(&mut self);
    fn get_value(&self)->u8;
    fn set_comment(&mut self,comment:String);
    fn clear_comment(&mut self);
    fn get_comment(&self)->Option<&String>;
    fn get_strand_name(&self)->&StrandName;
    fn load_saved(&mut self){
        let val = read_val_byte(self.get_strand_name().to_string_v().as_str());
        if (val & GRADE_EXISTS_FLAG)!=0 {
            log(&(val & !GRADE_EXISTS_FLAG).to_string());
            self.assign_value(val & !GRADE_EXISTS_FLAG);
        }
        let comment = read_val_string(self.get_strand_name().to_string());
        if comment.len() != 0 {
            log(&comment);
            self.set_comment(comment);
        }
    }
    fn save_comment(&self){
        save_val_string(
            self.get_strand_name().to_string(),
            self.get_comment().map_or("", |v| v)
        )
    }
    fn save_value(&self);
    fn on_comment_input(&mut self,comment: String){
        self.set_comment(comment);
        self.save_comment();
    }
    fn on_value_input(&mut self, val: u8){
        self.assign_value(val);
        self.save_value();
    }
    fn on_value_clear(&mut self){
        self.clear_value();
        self.save_comment();
    }
    fn on_comment_clear(&mut self){
        self.clear_comment();
        self.save_value();
    }
}

struct NumberedStrand{
    ui_component:HtmlComponent,
    comment:Option<String>,
    value:u8,
    strand_name:StrandName
}

impl NumberedStrand{
    fn new(strand_name: StrandName, html_parent: &HtmlComponent)->NumberedStrand{
        NumberedStrand{
            ui_component: 
                html_parent
                .create_child(
                    &SUB_STRAND_HTML
                    .replace("placeholder_strand", strand_name.to_string()), 
                    strand_name.to_string())
                .unwrap(),
            comment:None,
            value:0,
            strand_name
        }
    }
    //https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_animations/Using_CSS_animations
    fn init_html(&self){
        self.hide();
    }
    fn show(&self){
        self.ui_component.restyle("height:100%;--anim-duration:.2s;--anim-direction:forward;visibility:initial;--anim-name:dropdown-anim");
    }
    fn hide(&self){
        self.ui_component.restyle("height:0;--anim-duration:0;--anim-direction:unset;visibility:collapse;--anim-name:none");
    }
}

impl Grade for NumberedStrand {
    fn get_strand_name(&self)->&StrandName {
        &self.strand_name
    }
    fn assign_value(&mut self,val: u8) {
        self.value = val;
    }
    fn clear_value(&mut self) {
        self.value = 0;
    }
    fn get_value(&self)->u8 {
        return self.value;
    }
    fn set_comment(&mut self,comment:String) {
        self.comment = Some(comment);
    }
    fn clear_comment(&mut self) {
        self.comment = None;
    }
    fn get_comment(&self)->Option<&String> {
        return self.comment.as_ref();
    }
    fn save_value(&self) {
        save_val_byte(&self.get_strand_name().to_string_v(), self.value | GRADE_EXISTS_FLAG);
    }
}
struct OverallCriteria{
    ui_component:HtmlComponent,
    comment:Option<String>,
    override_val:Option<u8>,
    strands:Vec<NumberedStrand>,
    strand_name:StrandName
}

impl Grade for OverallCriteria {
    fn get_strand_name(&self)->&StrandName {
        &self.strand_name
    }
    fn assign_value(&mut self,val: u8) {
        self.override_val = Some(val);
    }
    fn clear_value(&mut self) {
        self.override_val = None;
    }
    fn get_value(&self)->u8 {
        let mut sum:f32 = 0.0;
        for strand in &self.strands {
            sum += strand.get_value() as f32;
        }
        sum /= self.strands.len() as f32;
        return self.override_val.unwrap_or(sum.round() as u8);
    }
    fn set_comment(&mut self,comment:String) {
        self.comment = Some(comment);
    }
    fn clear_comment(&mut self) {
        self.comment = None;
    }
    fn get_comment(&self)->Option<&String> {
        return self.comment.as_ref();
    }
    fn save_value(&self) {
        save_val_byte(&self.get_strand_name().to_string_v(), self.override_val.map_or(0, |v| v|GRADE_EXISTS_FLAG));
    }
}

impl OverallCriteria{
    fn new(strand_name:StrandName,ref_strands:Vec<NumberedStrand>,html_parent: &HtmlComponent)->OverallCriteria{
        let mut c = OverallCriteria{
            ui_component: 
                html_parent
                .create_child(
                    &OVERALL_STRAND_HTML
                    .replace("placeholder_strand", strand_name.to_string())
                    .replace(" Overall",""), 
                    strand_name.to_string())
                .unwrap(),
            comment:None,
            override_val:None,
            strands:ref_strands,
            strand_name
        };
        c.init_html();
        c
    }
    fn init_html(&self){
        for s in (0..self.strands.len()).rev(){
            let strand = &self.strands[s];
            strand.init_html();
            strand.ui_component.move_after(&self.ui_component);
        }
    }
    fn show_sub_strands(&self){
        for s in &self.strands{
            s.show();
        }
    }
    fn hide_sub_strands(&self){
        for s in &self.strands{
            s.hide();
        }
    }
}

struct HtmlComponent{
    html_id: String
}

impl HtmlComponent {
    fn new_from_document(id:String) -> Option<HtmlComponent>{
        if !check_component_exists(&id) {
            crash("attempted to get component from document which does not exist");
            return None;
        }
        Some(HtmlComponent{
            html_id:id
        })
    }
    fn create_child(&self,html: &str,id: &str) -> Option<HtmlComponent>{
        let html_id = id.to_string();
        if !extern_create_child(&self.html_id, html, &html_id){
            crash("failed to create child component");
            return None;
        }
        HtmlComponent::new_from_document(html_id)
    }
    fn move_after(&self, other: &HtmlComponent)-> bool{
        extern_move_after(&self.html_id, &other.html_id)
    }
    fn restyle(&self, style: &str)-> bool{
        extern_restyle(&self.html_id, style)
    }
    fn set_content_text(&self, text: &str){
        extern_set_content_text(&self.html_id,text);
    }
}

trait MouseEventListener{
    fn on_mouse_down(&mut self, id: &str, interface_id: &Vec<u8>);
    fn on_mouse_up(&mut self, id: &str, interface_id: &Vec<u8>);
}

#[wasm_bindgen]
struct MouseEventInterface{
    id:Vec<u8>,
    callback:Rc<RefCell<dyn MouseEventListener>> //where do I start?
    //Rc (reference count, lifetime guarentees may not be easily ensured by the lifetime of class it is stored in)
    //RefCell (indirection for reference???? idk, allows attempts to mutably borrow and may panic if borrows already exist, can only have one owner of the value, requiring reference counting to allow multuple ownership of data)
}

#[wasm_bindgen]
#[allow(dead_code)]//tell compiler this is used externally
impl MouseEventInterface {
    pub fn on_mouse_down(&self,id: &str){
        self.callback.borrow_mut().on_mouse_down(id, &self.id);
    }
    pub fn on_mouse_up(&self,id: &str){
        self.callback.borrow_mut().on_mouse_up(id, &self.id);
    }
}

//need to use refcell
impl MouseEventInterface {
    //callback needso to be a ref
    fn new(callback: Rc<RefCell<dyn MouseEventListener>>,id: Vec<u8>)->MouseEventInterface{
        MouseEventInterface{
            callback,
            id
        }
    }
}

trait InputEventListener{
    fn on_input(&mut self, id: &str, data: &str);
    fn on_change(&mut self, id: &str, data: &str);
}

#[wasm_bindgen]
struct InputEventInterface{
    callback:Rc<RefCell<dyn InputEventListener>>
}

#[wasm_bindgen]
#[allow(dead_code)]//tell compiler this is used externally
impl InputEventInterface{
    pub fn on_input(&self, id: &str, data: &str){
        self.callback.borrow_mut().on_input(id, data);
    }
    pub fn on_change(&self, id: &str, data: &str){
        self.callback.borrow_mut().on_change(id, data);
    }
}

impl InputEventInterface{
    fn new(callback:Rc<RefCell<dyn InputEventListener>>)-> InputEventInterface{
        InputEventInterface{
            callback
        }
    }
}