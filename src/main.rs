// #![windows_subsystem = "windows"]
// #![allow(unused)]
//
// use cpp_core::{Ptr, StaticUpcast};
// use qt_core::{
//     q_init_resource, qs, slot, CheckState, QBox, QObject, QPtr, QPointF, SlotNoArgs, QFlags, AlignmentFlag, QCoreApplication
// };
// use qt_ui_tools::ui_form;
// use qt_gui::{QPen, QBrush, QColor, q_painter::RenderHint};
// use qt_widgets::{QApplication, QPushButton, QWidget, QFrame, QVBoxLayout};
// use qt_charts::{QChart, QChartView, QLineSeries, QListOfQPointF, q_chart::AnimationOption, QValueAxis};
// use rand::prelude::*;
// use std::{thread, time, rc::Rc};
// use std::sync::mpsc::{self, channel, sync_channel, Receiver, Sender, TryRecvError};
// use std::sync::{Arc, Mutex};
// use std::time::{Duration, Instant};
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
//     series: QBox<QLineSeries>,
//     abort: Arc<Mutex<i32>>,
//     xaxis:  QBox<QValueAxis>,
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
//                 xaxis: QValueAxis::new_0a(),
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
//        self.chart.set_background_brush(&QBrush::from_q_color(&QColor::from_rgb_3a(0, 0, 0)));
//
//        self.chartview.set_chart(&self.chart);
//        self.chartview.set_window_title(&qs("Charts example"));
//        self.chartview.set_render_hint_1a(RenderHint::Antialiasing);
//        self.chartview.set_render_hint_1a(RenderHint::SmoothPixmapTransform);
//        self.chartview.set_render_hint_1a(RenderHint::HighQualityAntialiasing);
//
//        let pen = QPen::new();
//        pen.set_width_f(0.5);
//        pen.set_color(&QColor::from_rgb_3a(0, 255, 0));
//        self.series.set_pen(&pen);
//
//        // let xAxis = QValueAxis::new_0a();
//        self.series.attach_axis(&self.xaxis);
//        self.chart.set_axis_x_2a(&self.xaxis, &self.series);
//        self.xaxis.set_range(0 as f64, 10000 as f64);
//
//        let layout = QVBoxLayout::new_1a(&self.form.frame); // Placing the chartview in the vertical layout
//        layout.add_widget(&self.chartview);
//
//        self.form.start.clicked().connect(&self.slot_on_start_clicked());
//        self.form.stop.clicked().connect(&self.slot_on_stop_clicked());
//     }
//
//     #[slot(SlotNoArgs)]
//     unsafe fn on_start_clicked(self: &Rc<Self>)  {
//         let N: i64 = 10000; // number of points to measure
//         let (tx, rx) = mpsc::sync_channel(0);
//         *self.abort.lock().unwrap() = 0;
//
//         let sender = thread::spawn(move || { // no GUI stuffs here !!
//             loop { // for _ in 0..1000 {
//                let mut rng = rand::thread_rng();
//                let mut tx_arr: Vec<(f64,f64)> = Vec::with_capacity(N as usize); // let mut tx_arr = vec![vec![0.0f64; 2]; _ncol as usize];
//                for i in 0..N as usize { // (0..=1000).map(|x| x as f64 / 1.0).map(|x| (x, rng.gen()))
//                   tx_arr.push((i as f64, rng.gen())); // tx_arr[i][0] = i as f64; tx_arr[i][1] = rng.gen();
//                }
//                tx.send(tx_arr).unwrap(); // tx.send(tx_arr).expect("Unable to send on channel");
//                // thread::sleep(Duration::from_millis(10)); // we need to wait here, in order to give time to the rx to retrieve the data !! if not the consumed RAM increases.
//                // thread::sleep(Duration::from_micros(10000));
//             }
//         });
//
//         // let now = Instant::now();
//         'rx: for received in rx { // The receiver lives in the main Thread, GUI stuffs are performed here
//             if *self.abort.lock().unwrap() == 0 {
//                 // let now = Instant::now();
//                 self.xaxis.set_range(0 as f64, N as f64);
//                 let list = QListOfQPointF::new();
//                 list.reserve(N as i32);
//                 for j in 0..N as usize {
//                     // let point = QPointF::new_2a(received[j].0 / N as f64, received[j].1); //let point = QPointF::new_2a(received[j][0] / _ncol as f64, received[j][1]);
//                     let point = QPointF::new_2a(received[j].0, received[j].1);
//                     list.append_q_point_f(&point);
//                 }
//                 self.series.replace_q_list_of_q_point_f(&list);
//                 list.clear();
//                 QCoreApplication::process_events_0a();
//             }
//             else {
//                 break 'rx;
//             }
//        }
//        // let elapsed = now.elapsed();
//        // println!("Elapsed: {:?}", elapsed);
//        sender.join();
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

#![windows_subsystem = "windows"]
#![allow(unused)]

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{q_init_resource, qs, slot, CheckState, QBox, QObject, QPtr, QPointF, SlotNoArgs, QFlags, AlignmentFlag, QCoreApplication};
use qt_ui_tools::ui_form;
use qt_gui::{QPen, QBrush, QColor, q_painter::RenderHint};
use qt_widgets::{QApplication, QPushButton, QWidget, QFrame, QVBoxLayout};
use qt_charts::{QChart, QChartView, QLineSeries, QListOfQPointF, q_chart::AnimationOption, QValueAxis};
use rand::prelude::*;
use std::{thread, time, rc::Rc};
use std::sync::mpsc::{self, channel, sync_channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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
    series: QBox<QLineSeries>,
    abort: Arc<Mutex<i32>>,
    xaxis: QBox<QValueAxis>,
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
                xaxis: QValueAxis::new_0a(),
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
       self.chart.add_series(&self.series);
       self.chart.create_default_axes();
       self.chart.set_animation_options(QFlags::from(AnimationOption::NoAnimation));
       self.chart.set_background_brush(&QBrush::from_q_color(&QColor::from_rgb_3a(0, 0, 0)));

       self.chartview.set_chart(&self.chart);
       self.chartview.set_window_title(&qs("Charts example"));
       self.chartview.set_render_hint_1a(RenderHint::Antialiasing);
       self.chartview.set_render_hint_1a(RenderHint::SmoothPixmapTransform);
       self.chartview.set_render_hint_1a(RenderHint::HighQualityAntialiasing);

       let pen = QPen::new();
       pen.set_width_f(0.5);
       pen.set_color(&QColor::from_rgb_3a(0, 255, 0));
       self.series.set_pen(&pen);

       self.series.attach_axis(&self.xaxis);
       self.chart.set_axis_x_2a(&self.xaxis, &self.series);
       self.xaxis.set_range(0 as f64, 10000 as f64);

       let layout = QVBoxLayout::new_1a(&self.form.frame); // Placing the chartview in the vertical layout
       layout.add_widget(&self.chartview);

       self.form.start.clicked().connect(&self.slot_on_start_clicked());
       self.form.stop.clicked().connect(&self.slot_on_stop_clicked());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_start_clicked(self: &Rc<Self>)  {
        let N: i64 = 10000; // number of points to measure
        let (tx, rx) = mpsc::sync_channel(0);
        *self.abort.lock().unwrap() = 0;

        let sender = thread::spawn(move || { // no GUI stuffs here !!
            loop { // for _ in 0..1000 {
               let mut rng = rand::thread_rng();
               let list = QListOfQPointF::new();
               list.reserve(N as i32);
               for i in 0..N as usize {
                   let point = QPointF::new_2a(i as f64, rng.gen());
                   list.append_q_point_f(&point);
               }
               tx.send(list).unwrap();
               list.clear();
            }
        });

        'rx: for received in rx { // The receiver lives in the main Thread, GUI stuffs are performed here
            if *self.abort.lock().unwrap() == 0 {
                println!("{:?}", rx);
                // self.xaxis.set_range(0 as f64, N as f64);
                // self.series.replace_q_list_of_q_point_f(&rx);
                QCoreApplication::process_events_0a();
            }
            else {
                break 'rx;
            }
       }
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
