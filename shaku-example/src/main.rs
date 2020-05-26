use shaku::Component;
use shaku::ContainerBuilder;
use shaku::Interface;
use std::sync::Arc;
trait IOutput: Interface {
    fn write(&self, content: String);
}

trait IDateWriter: Interface {
    fn write_date(&self);
}

#[derive(Component)]
#[shaku(interface = IOutput)]
struct ConsoleOutput;

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!("{}", content);
    }
}

#[derive(Component)]
#[shaku(interface = IdateWriter)]
struct TodayWriter {
    #[shaku(inject)]
    output: Arc<dyn IOutput>,
    today: String,
    year: usize,
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        self.output
            .write(format!("Today is {}, {}", self.today, self.year));
    }
}
