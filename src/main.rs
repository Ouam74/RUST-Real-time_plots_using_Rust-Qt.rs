// #![windows_subsystem = "windows"]
// #![allow(unused)]
//
// use cpp_core::{Ptr, StaticUpcast};
// use qt_core::{
//     q_init_resource, qs, slot, CheckState, QBox, QObject, QPtr, QPointF,SlotNoArgs, QFlags, AlignmentFlag, QCoreApplication
// };
// use qt_ui_tools::ui_form;
// use qt_gui::q_painter::RenderHint;
// use qt_widgets::{QApplication, QPushButton, QWidget, QFrame, QVBoxLayout};
// use qt_charts::{QChart, QChartView, QLineSeries, QListOfQPointF};
// use qt_charts::q_chart::AnimationOption;
// use std::rc::Rc;
// use rand::prelude::*;
// use std::{thread, time};
// use std::sync::mpsc::channel;
// use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
// use std::sync::{Arc, Mutex};
// use std::time::Duration;
// use std::time::Instant;
//
//
// enum MyData {
//     DataArray(Vec<(f64, f64)>),
//     Message(String)
// }
//
//
// #[ui_form("../ui/SpecPXA_RUST.ui")]
// #[derive(Debug)]
// struct Form {
//     widget: QBox<QWidget>,
//     start: QPtr<QPushButton>,
//     stop: QPtr<QPushButton>,
//     frame: QPtr<QFrame>,
// }
//
// #[derive(Debug)]
// struct TodoWidget {
//     form: Form,
//     chart: QBox<QChart>,
//     chartview: QBox<QChartView>,
//     series:  QBox<QLineSeries>,
//     abort: Arc<Mutex<i32>>,
// }
//
// impl StaticUpcast<QObject> for TodoWidget {
//     unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
//         ptr.form.widget.as_ptr().static_upcast()
//     }
// }
//
// impl TodoWidget {
//
//     fn new() -> Rc<Self> {
//         unsafe {
//             let this = Rc::new(TodoWidget {
//                 form: Form::load(),
//                 chart: QChart::new_0a(),
//                 chartview: QChartView::new(),
//                 series: QLineSeries::new_0a(),
//                 abort: Arc::new(Mutex::new(0)),
//             });
//             this.init();
//             this
//         }
//     }
//
//     unsafe fn init(self: &Rc<Self>) {
//        self.chart.add_series(&self.series);
//        self.chart.create_default_axes();
//        self.chart.set_animation_options(QFlags::from(AnimationOption::NoAnimation));
//
//        self.chartview.set_chart(&self.chart);
//        self.chartview.set_window_title(&qs("Charts example"));
//        self.chartview.set_render_hint_1a(RenderHint::Antialiasing);
//
//        // Placing the chartview in the vertical layout
//        let layout = QVBoxLayout::new_1a(&self.form.frame);
//        layout.add_widget(&self.chartview);
//
//        self.form.start.clicked().connect(&self.slot_on_start_clicked());
//        self.form.stop.clicked().connect(&self.slot_on_stop_clicked());
//     }
//
//     #[slot(SlotNoArgs)]
//     unsafe fn on_start_clicked(self: &Rc<Self>) {
//         // let (tx, rx) = mpsc::channel(); channel::Vec<(f64,f64)>()
//         let _ncol: i64 = 1000; // number of points to measure
//         let (tx, rx) = mpsc::channel();
//         // let (tx, rx): (Sender<MyData>, Receiver<MyData>) = mpsc::channel();
//
//         *self.abort.lock().unwrap() = 0;
//
//         let abort_clone = Arc::clone(&self.abort); // let mut abort_clone = self.abort.clone();
//         let sender = thread::spawn(move || {
//             loop {
//                 let abort = abort_clone.lock().unwrap();
//                 if *abort == 0 {
//                    let mut rng = rand::thread_rng();
//                    let mut tx_arr: Vec<(f64,f64)> = Vec::with_capacity(_ncol as usize);
//                    for i in 0.._ncol { // (0..=1000).map(|x| x as f64 / 1.0).map(|x| (x, rng.gen()))
//                        tx_arr.push((i as f64, rng.gen()));
//                    }
//                    // thread::sleep(Duration::from_micros(1000)); // time to get the datas
//                    tx.send(tx_arr).unwrap();
//                    // tx.send(MyData::DataArray(tx_arr)).unwrap(); // send (emit) 'i' to channel, receiver will be run on the main thread
//                }
//                else {
//                    println!("sender break!");
//                    break;
//                }
//             }
//         });
//
//         loop { // loop runs in the main thread
//             match rx.recv() {
//                 Ok(_) => {
//                     let rx_array = rx.recv().unwrap();
//                     let now = Instant::now();
//
//                     let list = QListOfQPointF::new();
//                     // list.reserve(_ncol as i32);
//
//                     for j in 0.._ncol as usize {
//                         let p = QPointF::new_2a(rx_array[j].0 / _ncol as f64, rx_array[j].1);
//                         list.append_q_point_f(&p);
//                     }
//
//                     self.series.replace_q_list_of_q_point_f(&list);
//                     QCoreApplication::process_events_0a();
//                     // thread::sleep(Duration::from_micros(1000)); // time to get the datas
//
//                     let elapsed = now.elapsed();
//                     // println!("Elapsed: {:?}", elapsed);
//                }
//                Err(_) => {
//                     println!("break!");
//                     break;
//                }
//            }
//        }
//        sender.join().unwrap();
//     }
//
//     #[slot(SlotNoArgs)]
//     unsafe fn on_stop_clicked(self: &Rc<Self>) {
//         *self.abort.lock().unwrap() = 1;
//         // println!("{:}", self.abort.lock().unwrap());
//     }
//
//     fn show(self: &Rc<Self>) {
//         unsafe {
//             self.form.widget.show();
//         }
//     }
// }
//
// fn main() {
//     QApplication::init(|_| {
//         let todo_widget = TodoWidget::new();
//         todo_widget.show();
//         unsafe { QApplication::exec() }
//     })
// }


//////////////////////////////////////////////////////////////////////////////////////////////////
#![windows_subsystem = "windows"]
#![allow(unused)]

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
    q_init_resource, qs, slot, CheckState, QBox, QObject, QPtr, QPointF,SlotNoArgs, QFlags, AlignmentFlag, QCoreApplication
};
use qt_ui_tools::ui_form;
use qt_gui::{QPen, QBrush, QColor};
use qt_gui::q_painter::RenderHint;
use qt_widgets::{QApplication, QPushButton, QWidget, QFrame, QVBoxLayout};
use qt_charts::{QChart, QChartView, QLineSeries, QListOfQPointF};
use qt_charts::q_chart::AnimationOption;
use std::rc::Rc;
use rand::prelude::*;
use std::{thread, time};
use std::sync::mpsc::channel;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::Instant;
use std::error::Error;

#[ui_form("../ui/SpecPXA_RUST.ui")]
#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    start: QPtr<QPushButton>,
    stop: QPtr<QPushButton>,
    frame: QPtr<QFrame>,
}

#[derive(Debug)]
struct TodoWidget {
    form: Form,
    chart: QBox<QChart>,
    chartview: QBox<QChartView>,
    series:  QBox<QLineSeries>,
    abort: Arc<Mutex<i32>>,
}

impl StaticUpcast<QObject> for TodoWidget {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl TodoWidget {

    fn new() -> Rc<Self> {
        unsafe {
            let this = Rc::new(TodoWidget {
                form: Form::load(),
                chart: QChart::new_0a(),
                chartview: QChartView::new(),
                series: QLineSeries::new_0a(),
                abort: Arc::new(Mutex::new(0)),
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
       self.chart.add_series(&self.series);
       self.chart.create_default_axes();
       self.chart.set_animation_options(QFlags::from(AnimationOption::NoAnimation));

       self.chartview.set_chart(&self.chart);
       self.chartview.set_window_title(&qs("Charts example"));
       self.chartview.set_render_hint_1a(RenderHint::Antialiasing);

       let pen = QPen::new();
       pen.set_width_f(0.5);
       pen.set_color(&QColor::from_rgb_3a(255, 0, 0));
       self.series.set_pen(&pen);

       let layout = QVBoxLayout::new_1a(&self.form.frame); // Placing the chartview in the vertical layout
       layout.add_widget(&self.chartview);

       self.form.start.clicked().connect(&self.slot_on_start_clicked());
       self.form.stop.clicked().connect(&self.slot_on_stop_clicked());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_start_clicked(self: &Rc<Self>)  {
        let _ncol: i64 = 1000; // number of points to measure
        let (tx, rx) = mpsc::channel();
        *self.abort.lock().unwrap() = 0;

        let sender = thread::spawn(move || {
            loop { // for _ in 0..1000 {
               let mut rng = rand::thread_rng();
               let mut tx_arr: Vec<(f64,f64)> = Vec::with_capacity(_ncol as usize); // let mut tx_arr = vec![vec![0.0f64; 2]; _ncol as usize];
               for i in 0.._ncol as usize { // (0..=1000).map(|x| x as f64 / 1.0).map(|x| (x, rng.gen()))
                  tx_arr.push((i as f64, rng.gen())); // tx_arr[i][0] = i as f64; tx_arr[i][1] = rng.gen();
               }
               tx.send(tx_arr).unwrap(); // tx.send(tx_arr).expect("Unable to send on channel");
               thread::sleep(Duration::from_millis(20)); // we need to wait here, in order to give time to the rx to retrieve the data !! if not the consumed RAM increases.
            }
        });

        // let now = Instant::now();
        'rx: for received in rx {
            if *self.abort.lock().unwrap() == 0 {
                // let now = Instant::now();
                let list = QListOfQPointF::new();
                list.reserve(_ncol as i32);
                for j in 0.._ncol as usize {
                    let point = QPointF::new_2a(received[j].0 / _ncol as f64, received[j].1); //let point = QPointF::new_2a(received[j][0] / _ncol as f64, received[j][1]);
                    list.append_q_point_f(&point);
                }
                self.series.replace_q_list_of_q_point_f(&list);
                QCoreApplication::process_events_0a();
            }
            else {
                break 'rx;
            }
       }
       // let elapsed = now.elapsed();
       // println!("Elapsed: {:?}", elapsed);
       sender.join();
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_stop_clicked(self: &Rc<Self>) {
        *self.abort.lock().unwrap() = 1;
        // println!("{:}", self.abort.lock().unwrap());
    }

    fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }
}

fn main() {
    QApplication::init(|_| {
        let todo_widget = TodoWidget::new();
        todo_widget.show();
        unsafe { QApplication::exec() }
    })
}
