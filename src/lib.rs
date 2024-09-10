
use core::panic;
use std::{cell::RefCell, rc::Rc, usize, vec};

use lopdf::{content::{Content, Operation}, dictionary, Document, Object, Stream};
use wasm_bindgen::prelude::*;

static GRADE_EXISTS_FLAG:u8 = 0b10000000;
static GRADE_SELECTED_VALUE_SLIDEBAR_ID:&str = "selected_value_slidebar";
static COMMENT_VALUE_INPUT_AREA_ID:&str = "comment_input";
static LEVEL_DESCRIPTOR_AREA_ID:&str = "level_descriptor";
static RESET_VALUE_BUTTON_ID:&str = "selected_value_reset";

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
    pub fn extern_listen_for_window_events(listener: WindowEventInterface);
    pub fn extern_get_input_value(id: &str) -> String;
    pub fn extern_set_content_text(id: &str, text: &str);
    pub fn extern_set_input_value(id: &str, value: &str);
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
    pub fn save_pdf(f: Vec<u8>);
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
        self.student_grades = Some(grades);
    }
    pub fn write_grade(&mut self,index: usize, val: u8){
        save_val_byte(&StrandName::from_index(index).to_string_v(), val | GRADE_EXISTS_FLAG)
    }
    pub fn generate_pdf(&self){

        // `with_version` specifes the PDF version this document complies with.
        let mut doc = Document::with_version("1.5");
        // Object IDs are used for cross referencing in PDF documents.
        // `lopdf` helps keep track of them for us. They are simple integers.
        // Calls to `doc.new_object_id` and `doc.add_object` return an object ID.

        // "Pages" is the root node of the page tree.
        let pages_id = doc.new_object_id();

        // Fonts are dictionaries. The "Type", "Subtype" and "BaseFont" tags
        // are straight out of the PDF spec.
        //
        // The dictionary macro is a helper that allows complex
        // key-value relationships to be represented in a simpler
        // visual manner, similar to a match statement.
        // A dictionary is implemented as an IndexMap of Vec<u8>, and Object
        let font_id = doc.add_object(dictionary! {
            // type of dictionary
            "Type" => "Font",
            // type of font, type1 is simple postscript font
            "Subtype" => "Type1",
            // basefont is postscript name of font for type1 font.
            // See PDF reference document for more details
            "BaseFont" => "Arial",
        });

        // Font dictionaries need to be added into resource
        // dictionaries in order to be used.
        // Resource dictionaries can contain more than just fonts,
        // but normally just contains fonts.
        // Only one resource dictionary is allowed per page tree root.
        let resources_id = doc.add_object(dictionary! {
            // Fonts are actually triplely nested dictionaries. Fun!
            "Font" => dictionary! {
                // F1 is the font name used when writing text.
                // It must be unique in the document. It does not
                // have to be F1
                "F1" => font_id,
            },
        });

        // `Content` is a wrapper struct around an operations struct that contains
        // a vector of operations. The operations struct contains a vector of
        // that match up with a particular PDF operator and operands.
        // Refer to the PDF spec for more details on the operators and operands
        // Note, the operators and operands are specified in a reverse order
        // from how they actually appear in the PDF file itself.
        let content = Content {
            operations: vec![
                // BT begins a text element. It takes no operands.
                Operation::new("BT", vec![]),
                // Tf specifies the font and font size.
                // Font scaling is complicated in PDFs.
                // Refer to the spec for more info.
                // The `into()` methods convert the types into
                // an enum that represents the basic object types in PDF documents.
                Operation::new("Tf", vec!["F1".into(), 15.into()]),
                // Td adjusts the translation components of the text matrix.
                // When used for the first time after BT, it sets the initial
                // text position on the page.
                // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
                Operation::new("Td", vec![50.into(), 800.into()]),
                // Tj prints a string literal to the page. By default, this is black text that is
                // filled in. There are other operators that can produce various textual effects and
                // colors
                Operation::new("Tj", vec![Object::string_literal("owo what is this a a a a a aaa aaaa aaaaaaaa a a a  a  a  a   a   a      a   a a aa a a a a")]),
                // ET ends the text element.
                Operation::new("ET", vec![]),


                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 15.into()]),
                Operation::new("Td", vec![50.into(), 780.into()]),
                Operation::new("Tj", vec![Object::string_literal("owo what is this a a a a a aaa aaaa aaaaaaaa a a a  a  a  a   a   a      a   a a aa a a a a")]),
                Operation::new("ET", vec![]),
            ],
        };

        // Streams are a dictionary followed by a (possibly encoded) sequence of bytes.
        // What that sequence of bytes represents, depends on the context.
        // The stream dictionary is set internally by lopdf and normally doesn't
        // need to be manually manipulated. It contains keys such as
        // Length, Filter, DecodeParams, etc.
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

        // Page is a dictionary that represents one page of a PDF file.
        // Its required fields are "Type", "Parent" and "Contents".
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        });

        // Again, "Pages" is the root of the page tree. The ID was already created
        // at the top of the page, since we needed it to assign to the parent element
        // of the page dictionary.
        //
        // These are just the basic requirements for a page tree root object.
        // There are also many additional entries that can be added to the dictionary,
        // if needed. Some of these can also be defined on the page dictionary itself,
        // and not inherited from the page tree root.
        let pages = dictionary! {
            // Type of dictionary
            "Type" => "Pages",
            // Vector of page IDs in document. Normally would contain more than one ID
            // and be produced using a loop of some kind.
            "Kids" => vec![page_id.into()],
            // Page count
            "Count" => 1,
            // ID of resources dictionary, defined earlier
            "Resources" => resources_id,
            // A rectangle that defines the boundaries of the physical or digital media.
            // This is the "page size".
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        };

        // Using `insert()` here, instead of `add_object()` since the ID is already known.
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        // Creating document catalog.
        // There are many more entries allowed in the catalog dictionary.
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        // The "Root" key in trailer is set to the ID of the document catalog,
        // the remainder of the trailer is set during `doc.save()`.
        doc.trailer.set("Root", catalog_id);
        doc.compress();

        let mut write:Vec<u8> = vec![];

        let res = doc.save_to(&mut write);
        if res.is_err(){
            log("failed to write pdf to buffer, help!");
            log(&res.unwrap_err().to_string());
            return;
        }
        save_pdf(write);
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
        let instance: Rc<RefCell<StudentGrades>> = Rc::new(RefCell::new(StudentGrades{
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
        for s in &instance.borrow().overall{
            extern_listen_for_mouse(&s.ui_component.html_id,MouseEventInterface::new(instance.clone(),vec![c1]));
            let mut c2 = 0;
            for c in &s.strands{
                extern_listen_for_mouse(&c.ui_component.html_id, MouseEventInterface::new(instance.clone(),vec![c1,c2]));
                c2+=1;
            }
            c1+=1;
        }
        extern_listen_for_mouse(RESET_VALUE_BUTTON_ID, MouseEventInterface::new(instance.clone(), vec![]));

        extern_listen_for_input(GRADE_SELECTED_VALUE_SLIDEBAR_ID, InputEventInterface::new(instance.clone()));
        extern_listen_for_input(COMMENT_VALUE_INPUT_AREA_ID, InputEventInterface::new(instance.clone()));
        extern_listen_for_window_events(WindowEventInterface::new(instance.clone()));

        instance.borrow_mut().load_last_grades();
        let id = instance.borrow().overall.get(0).map(|v|&v.ui_component.html_id).unwrap().to_owned();
        extern_simulate_click(&id);
        instance
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
        self.unselect_substrand();
        self.show_overall_on_display(&self.overall[i]);
    }
    fn unselect_overall(&mut self){
        if self.current_selected_overall.is_none(){
            return;
        }
        self.overall[self.current_selected_overall.unwrap()].hide_sub_strands();
        self.current_selected_overall = None;
    }
    fn unselect_substrand(&mut self){
        self.last_displayed_substrand = None;
        extern_restyle(LEVEL_DESCRIPTOR_AREA_ID, "visibility: collapse;height:0;");
    }
    fn select_substrand(&mut self, substrand_id: &Vec<u8>) {
        let overall_idx = *substrand_id.get(0).unwrap() as usize;
        let sub_idx = *substrand_id.get(1).unwrap() as usize;

        self.last_displayed_substrand = Some(substrand_id.to_owned());

        let strand = self.overall
            .get(overall_idx).unwrap()
            .strands.get(sub_idx).unwrap();
        
        self.show_substrand_on_display(strand);
    }
    fn show_substrand_on_display(&self, strand: &NumberedStrand) {
        extern_restyle(LEVEL_DESCRIPTOR_AREA_ID, "visibility: unset;");
        let criteria = strand.get_strand_name().to_criteria();
        for i in 0..usize::min(criteria.len(),self.strand_display_elements.len()){
            self.strand_display_elements[i].set_content_text(criteria[i]);
        }
        extern_set_input_value(GRADE_SELECTED_VALUE_SLIDEBAR_ID, &strand.get_value().to_string());
        strand.get_comment()
            .map_or_else(
                ||extern_set_input_value(COMMENT_VALUE_INPUT_AREA_ID, ""),
                |v|extern_set_input_value(COMMENT_VALUE_INPUT_AREA_ID, v)
            );
    }
    fn show_overall_on_display(&self, strand: &OverallCriteria){
        extern_set_input_value(GRADE_SELECTED_VALUE_SLIDEBAR_ID, &strand.get_value().to_string());
        strand.get_comment()
            .map_or_else(
                ||extern_set_input_value(COMMENT_VALUE_INPUT_AREA_ID, ""),
                |v|extern_set_input_value(COMMENT_VALUE_INPUT_AREA_ID, v)
            );
    }
}

impl MouseEventListener for StudentGrades{
    fn on_mouse_down(&mut self, _id: &str, interface_id: &Vec<u8>) {
        if interface_id.len() == 0 {
            if self.last_displayed_substrand.is_none(){
                let overall = self.overall.get_mut(self.current_selected_overall.unwrap()).unwrap();
                overall.on_value_clear();
                extern_set_input_value(GRADE_SELECTED_VALUE_SLIDEBAR_ID, &overall.get_value().to_string());
            }else{
                let substrand_id = self.last_displayed_substrand.as_ref().unwrap();
                let overall = self.overall.get_mut(substrand_id.get(0).unwrap_or(&0).to_owned() as usize).unwrap();
                let sub = overall.strands.get_mut(substrand_id.get(1).unwrap_or(&0).to_owned() as usize).unwrap();
                sub.on_value_clear();
                extern_set_input_value(GRADE_SELECTED_VALUE_SLIDEBAR_ID, &sub.get_value().to_string());
            }
            return;
        }
        let overall_idx = *interface_id.get(0).unwrap() as usize;
        if interface_id.len() == 1 { // if is strand
            if self.current_selected_overall.is_some() {
                self.unselect_overall();
            }
            self.select_overall(overall_idx);
        }else if interface_id.len() == 2 {  // if is substrand
            self.select_substrand(interface_id);
        }
    }
    fn on_mouse_up(&mut self, _id: &str, _interface_id: &Vec<u8>) {
        
    }
}

impl WindowEventListener for StudentGrades {
    fn on_window_change(&mut self) {

    }
    fn on_window_close(&mut self) {
        
    }
}

impl InputEventListener for StudentGrades{
    fn on_input(&mut self, id: &str, data: &str) {
        log(&(id.to_owned() + "changed"));
        if self.last_displayed_substrand.is_none(){
            if self.current_selected_overall.is_none(){
                return;
            }
            let overall = self.overall.get_mut(self.current_selected_overall.unwrap()).unwrap();
            if id == GRADE_SELECTED_VALUE_SLIDEBAR_ID {
                let res = u8::from_str_radix(data, 10);
                if res.is_ok(){
                    let g = res.unwrap();
                    overall.on_value_input(g);
                }
            }else if id == COMMENT_VALUE_INPUT_AREA_ID {
                if data.len()>0 {
                    overall.on_comment_input(data.to_string());
                }else{
                    overall.on_comment_clear();
                }
            }
            return;
        }
        let substrand_id = self.last_displayed_substrand.as_ref().unwrap();
        let overall = self.overall.get_mut(substrand_id.get(0).unwrap_or(&0).to_owned() as usize).unwrap();
        let sub = overall.strands.get_mut(substrand_id.get(1).unwrap_or(&0).to_owned() as usize).unwrap();
        if id == GRADE_SELECTED_VALUE_SLIDEBAR_ID {
            let res = u8::from_str_radix(data, 10);
            if res.is_ok(){
                let g = res.unwrap();
                sub.on_value_input(g);
            }
        }else if id == COMMENT_VALUE_INPUT_AREA_ID {
            if data.len()>0 {
                sub.on_comment_input(data.to_string());
            }else{
                sub.on_comment_clear();
            }
        }
    }
    fn on_change(&mut self, _id: &str, _data: &str){
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
    fn on_comment_clear(&mut self){
        self.clear_comment();
        self.save_comment();
    }
    fn on_value_clear(&mut self){
        self.clear_value();
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
        let c = OverallCriteria{
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

trait WindowEventListener{
    fn on_window_change(&mut self);
    fn on_window_close(&mut self);
}

#[wasm_bindgen]
struct WindowEventInterface{
    callback:Rc<RefCell<dyn WindowEventListener>>
}

#[wasm_bindgen]
#[allow(dead_code)]
impl WindowEventInterface{
    pub fn on_window_change(&self){
        self.callback.borrow_mut().on_window_change();
    }
    pub fn on_window_close(&self){
        self.callback.borrow_mut().on_window_close();
    }
}

impl WindowEventInterface{
    fn new(callback: Rc<RefCell<dyn WindowEventListener>>)->WindowEventInterface{
        WindowEventInterface{
            callback
        }
    }
}