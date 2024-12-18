// 定义一个特征（Trait），用于约束可以作为成绩类型的数据，它们需要实现Display特征以便格式化输出
trait GradeType: std::fmt::Display {}

// 为f32类型实现GradeType特征，使其能作为合法的成绩类型（数字成绩）
impl GradeType for f32 {}

// 为&str类型实现GradeType特征，使其能作为合法的成绩类型（字母成绩）
impl GradeType for &str {}

// ReportCard结构体使用泛型T，并且要求T必须实现GradeType特征
struct ReportCard<T: GradeType> {
    grade: T,
    student_name: String,
    student_age: u8,
}

impl<T: GradeType> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

fn main() {
    // 可以在这里进行一些自定义的测试或者使用示例
    let numeric_report_card = ReportCard {
        grade: 3.5,
        student_name: "Alice".to_string(),
        student_age: 13,
    };
    println!("{}", numeric_report_card.print());

    let alphabetic_report_card = ReportCard {
        grade: "B",
        student_name: "Bob".to_string(),
        student_age: 12,
    };
    println!("{}", alphabetic_report_card.print());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}