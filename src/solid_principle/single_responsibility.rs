pub struct EmployeeData {
    pub name: String,
    pub deparment: String,
}

impl EmployeeData {
    pub fn new(name: String, deparment: String) -> Self {
        Self {
            name,
            deparment,
        }
    }
}

pub struct PayCalculator;

impl PayCalculator {
    pub fn new() -> Self {
        Self {}
    }

    fn get_regular_hours() {
        println!("給与計算専用の労働時間の計算ロジック")
    }

    pub fn calculate_pay(&self, employee_data: &EmployeeData) {
        Self::get_regular_hours();
        println!("{}の給与を計算しました", employee_data.name);
    }
}

pub struct HourReporter;

impl HourReporter {
    pub fn new() -> Self {
        Self {}
    }

    fn get_regular_hours() {
        println!("給与計算専用の労働時間の計算ロジック_V2")
    }

    pub fn repot_hours(&self, employee_data: &EmployeeData) {
        Self::get_regular_hours();
        println!("{}の労働時間をレポートしました", employee_data.name);
    }
}

pub struct Main;

impl Main {
    pub fn index() {
        let employee_data = EmployeeData::new("Suzuki".to_string(), "develop".to_string());

        println!("経理部門");
        let pay_caluculaor = PayCalculator::new();
        pay_caluculaor.calculate_pay(&employee_data);

        println!("人事部門");
        let hour_reporter = HourReporter::new();
        hour_reporter.repot_hours(&employee_data);
    }
}
