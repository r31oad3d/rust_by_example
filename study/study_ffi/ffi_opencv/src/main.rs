use opencv::{
    core, highgui, imgcodecs, imgcodecs::IMREAD_COLOR, imgproc, prelude::*,
    videoio,
};

fn main() {
    fn cv_demo() {
        let window = "video capture1";
        highgui::named_window(window, 1);
        let mat = imgcodecs::imread(
            "/Users/johnj/Desktop/Screen Shot 2020-06-22 at 21.50.31.png",
            IMREAD_COLOR,
        );
        mat.map(|m| {
            highgui::imshow("rust", &m);
            highgui::wait_key(0);
            highgui::destroy_window("name");
        });
    }

    cv_demo();
}
