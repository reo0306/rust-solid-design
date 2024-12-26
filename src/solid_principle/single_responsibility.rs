#[derive(Clone)]
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

pub struct PayCalculator {
    pub employee_data: EmployeeData,
}

impl PayCalculator {
    pub fn new(employee_data: EmployeeData) -> Self {
        Self { employee_data }
    }

    fn get_regular_hours() {
        println!("給与計算専用の労働時間の計算ロジック")
    }

    pub fn calculate_pay(&self) {
        Self::get_regular_hours();
        println!("{}の給与を計算しました", self.employee_data.name);
    }
}

pub struct HourReporter {
    pub employee_data: EmployeeData,
}

impl HourReporter {
    pub fn new(employee_data: EmployeeData) -> Self {
        Self { employee_data }
    }

    fn get_regular_hours() {
        println!("給与計算専用の労働時間の計算ロジック_V2")
    }

    pub fn repot_hours(&self) {
        Self::get_regular_hours();
        println!("{}の労働時間をレポートしました", self.employee_data.name);
    }
}

pub struct Main;

impl Main {
    pub fn index() {
        let employee_data = EmployeeData::new("Suzuki".to_string(), "develop".to_string());

        println!("経理部門");
        let pay_caluculaor = PayCalculator::new(employee_data.clone());
        pay_caluculaor.calculate_pay();

        println!("人事部門");
        let hour_reporter = HourReporter::new(employee_data.clone());
        hour_reporter.repot_hours();
    }
}
