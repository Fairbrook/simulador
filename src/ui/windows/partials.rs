use nwg::stretch::{
    geometry::{Size},
    style::{Dimension as D, FlexDirection}
};
use nwd::NwgPartial;
use crate::types::{process::{StatefulProcess, State}, seconds_to_str};

#[derive(Default, NwgPartial)]
pub struct ReadyQueue {
    #[nwg_layout(flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: "Procesos en cola")]
    #[nwg_layout_item(layout: layout, size:Size{width: D::Auto, height: D::Percent(0.05)})]
    label: nwg::Label,

    #[nwg_control(item_count: 200,  list_style: nwg::ListViewStyle::Detailed, focus: true,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT, 
    )]  
    #[nwg_layout_item(layout: layout,size:Size{width: D::Auto, height: D::Percent(0.95)})]
    data_view: nwg::ListView,
}

impl ReadyQueue{
    pub fn setup(&self){
        let dv = &self.data_view;
        dv.insert_column("ID");
        dv.insert_column("Estimado");
        dv.insert_column("Restante");
        dv.set_headers_enabled(true);
    }
    pub fn set_list(&self, list: &Vec<StatefulProcess>){
        let dv = &self.data_view;
        dv.clear();

        for state in list{
            dv.insert_items_row(None, &[state.process.pid.clone(),state.process.et.to_string(), (state.process.et-state.times.service_seconds).to_string()])
        }
    }
}

#[derive(Default, NwgPartial)]
pub struct BlockedQueue {
    #[nwg_layout(flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: "Procesos bloqueados")]
    #[nwg_layout_item(layout: layout, size:Size{width: D::Auto, height: D::Percent(0.05)})]
    label: nwg::Label,

    #[nwg_control(item_count: 200,  list_style: nwg::ListViewStyle::Detailed, focus: true,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT, 
    )]  
    #[nwg_layout_item(layout: layout,size:Size{width: D::Auto, height: D::Percent(0.95)})]
    data_view: nwg::ListView,
}

impl BlockedQueue{
    pub fn setup(&self){
        let dv = &self.data_view;
        dv.insert_column("ID");
        dv.insert_column("Tiempo transcurrido");
        dv.set_headers_enabled(true);
    }

    pub fn set_list(&self, list: &Vec<StatefulProcess>){
        let dv = &self.data_view;
        dv.clear();

        for state in list{
            dv.insert_items_row(None, &[state.process.pid.clone(),seconds_to_str(state.times.blocked_seconds)])
        }
    }
}

#[derive(Default, NwgPartial)]
pub struct FinishedList {
    #[nwg_layout(flex_direction: FlexDirection::Column)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(text: "Procesos Terminados")]
    #[nwg_layout_item(layout: layout, size:Size{width: D::Auto, height: D::Percent(0.05)})]
    label: nwg::Label,

    #[nwg_control(item_count: 200,  list_style: nwg::ListViewStyle::Detailed, focus: true,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT, 
    )]  
    #[nwg_layout_item(layout: layout,size:Size{width: D::Auto, height: D::Percent(0.95)})]
    data_view: nwg::ListView,
}

impl FinishedList{
    pub fn setup(&self){
        let dv = &self.data_view;
        dv.insert_column("ID");
        dv.insert_column("Operación");
        dv.insert_column("Resultado");
        dv.set_headers_enabled(true);
    }

    pub fn set_list(&self, list: &Vec<StatefulProcess>){
        let dv = &self.data_view;
        dv.clear();

        for state in list{
            let res = match state.state{
                State::Error=>String::from("Error"),
                State::Finished=>state.result.to_string(),
                _=>String::from("")
            };
            dv.insert_items_row(None, &[state.process.pid.clone(),state.process.operation.to_string(), res])
        }
    }
}

#[derive(Default, NwgPartial)]
pub struct Runing{
    #[nwg_layout(spacing: 2)]
    layout: nwg::GridLayout,

    #[nwg_control(text:"PID: ", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:0,col:0)]
    pid_label: nwg::Label,

    #[nwg_control(text:"", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:0,col:1, col_span: 2)]
    pid: nwg::Label,

    #[nwg_control(text:"Operación: ", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:1,col:0)]
    operation_label: nwg::Label,

    #[nwg_control(text:"00:00", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:1,col:1, col_span: 2)]
    operation: nwg::Label,

    #[nwg_control(text:"Transcurrido: ", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:2,col:0)]
    elapsed_label: nwg::Label,

    #[nwg_control(text:"00:00", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:2,col:1, col_span: 2)]
    elapsed: nwg::Label,

    #[nwg_control(text:"Restante: ", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:3,col:0)]
    remaning_label: nwg::Label,

    #[nwg_control(text:"00:00", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:3,col:1, col_span: 2)]
    remaning: nwg::Label,
}

impl Runing{
    pub fn upate(&self, state_option: Option<StatefulProcess>){
        match state_option{
            None=>{
                self.elapsed.set_text("");
                self.operation.set_text("");
                self.pid.set_text("");
                self.remaning.set_text("");
            },
            Some(state)=>{
                self.elapsed.set_text(&seconds_to_str(state.times.service_seconds)[..]);
                self.remaning.set_text(&seconds_to_str(state.process.et - state.times.service_seconds)[..]);
                self.pid.set_text(&state.process.pid[..]);
                self.operation.set_text(&state.process.operation.to_string()[..]);
            }
        }
    }
}