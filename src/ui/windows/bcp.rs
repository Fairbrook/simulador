use nwd::NwgUi;
use nwg::NativeUi;

pub struct BCPDialog{
    window: nwg::Window,
    labels: RefCell<Vec<nwg::Label>>
}