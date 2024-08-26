
use core::panic;
use std::usize;

use wasm_bindgen::prelude::*;

static GRADE_EXISTS_FLAG:u8 = 0b10000000;
#[wasm_bindgen]
extern {
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
    student_grades:Option<StudentGrades>
}

#[wasm_bindgen]
impl State {
    pub fn load_last_grades(&mut self){
        let mut grades = StudentGrades::new();
        grades.load_last_grades();
        self.student_grades = Some(grades);
    }
    pub fn write_grade(&mut self,index: usize, val: u8){
        save_val_byte(&StrandName::from_index(index).to_string_v(), val | GRADE_EXISTS_FLAG)
    }
}

pub struct StudentGrades{
    overall:Vec<OverallCriteria>
}

impl StudentGrades {
    fn new()->StudentGrades{
        let overall_criteria = vec![
            OverallCriteria::new(StrandName::ResearchDesignOverall,vec![
                NumberedStrand::new(StrandName::ResearchDesign1),
                NumberedStrand::new(StrandName::ResearchDesign2),
                NumberedStrand::new(StrandName::ResearchDesign3)
            ]),
            OverallCriteria::new(StrandName::DataAnalysisOverall,vec![
                NumberedStrand::new(StrandName::DataAnalysis1),
                NumberedStrand::new(StrandName::DataAnalysis2),
                NumberedStrand::new(StrandName::DataAnalysis3)
            ]),
            OverallCriteria::new(StrandName::ConclusionOverall,vec![
                NumberedStrand::new(StrandName::Conclusion1),
                NumberedStrand::new(StrandName::Conclusion2)
            ]),
            OverallCriteria::new(StrandName::EvaluationOverall,vec![
                NumberedStrand::new(StrandName::Evaluation1),
                NumberedStrand::new(StrandName::Evaluation2)
            ])
        ];
        StudentGrades{
            overall:overall_criteria
        }
    }
    fn load_last_grades(&mut self){
        for strand in &mut self.overall{
            strand.load_saved();
            for sub_strand in &mut strand.strands{
                sub_strand.load_saved();
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
            Self::Evaluation2 => "Evaluation 2",
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
    comment:Option<String>,
    value:u8,
    strand_name:StrandName
}

impl NumberedStrand{
    fn new(strand_name: StrandName)->NumberedStrand{
        NumberedStrand{
            comment:None,
            value:0,
            strand_name
        }
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
    fn new(strand_name:StrandName,ref_strands:Vec<NumberedStrand>)->OverallCriteria{
        OverallCriteria{
            comment:None,
            override_val:None,
            strands:ref_strands,
            strand_name
        }
    }
}